use crate::auth::g_auth::*;
use crate::auth::utils::random_session_token;
use crate::*;
use actix_web::cookie::Cookie;
use actix_web::*;
use reqwest::{Client, Url};
use std::error::Error;

/// Redirects the user to the google login page
#[get("/api/login")]
async fn login(req: HttpRequest, data: web::Data<ServerData>) -> impl Responder {
    let client_id = &data.auth.client_id;
    let redirect = &data.auth.redirect_url;

    let url = gen_login_url(&client_id, &redirect);

    // Redirect the user to the Google login URI
    HttpResponse::Found().header("Location", url).finish()
}

/// Endpoint after the user signs in with Google
#[get("/api/g_auth")]
async fn login_fallback(
    query: web::Query<QueryCode>,
    data: web::Data<ServerData>,
) -> impl Responder {
    let code = &query.code;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = request_token(code.as_str(), &data).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();

    let access_token = token_response.access_token;
    let id_token = token_response.id_token;

    let google_user = get_google_user(&access_token, &id_token).await;
    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let google_user = google_user.unwrap();

    // Generate a random session token
    let token = random_session_token();
    // Give token as a cookies
    let cookie = Cookie::build("token", &token).path("/").finish();

    if data.db.user_id_exists(&google_user.id) {
        // user has an account

        return HttpResponse::Found()
            .cookie(cookie)
            .header("Location", "/")
            .finish();
    } else {
        // User does not have an account

        // Structure to remember this username
        // when he pressed the register button
        let unregistered_user = UnregisteredUser {
            google_user,
            access_token,
            id_token,
        };

        // Add unregistered user to the map
        data.add_unregistered_user(token.clone(), unregistered_user);

        return HttpResponse::Found()
            .cookie(cookie)
            .header("Location", "/register")
            .finish();
    }
}

#[derive(serde::Deserialize)]
struct RegisterForm {
    username: String,
}

/// Register an account
#[post("/api/register")]
async fn register(
    req: HttpRequest,
    data: web::Data<ServerData>,
    form: web::Form<RegisterForm>,
) -> impl Responder {
    let username = form.username.clone();

    // Retrieve token from cookie
    let token = match req.cookie("token") {
        Some(token_cookie) => token_cookie.value().to_string(),
        None => {
            return HttpResponse::BadGateway().json(
                serde_json::json!({"status": "fail", "message": "You are not authenticated"}),
            );
        }
    };

    // check if user is authenticated
    let user = match data.take_unregistered_user(token.clone()) {
        Some(user) => user,
        None => {
            return HttpResponse::BadGateway().json(
                serde_json::json!({"status": "fail", "message": "You are not authenticated"}),
            );
        }
    };

    // check if username already exists
    if data.db.username_exists(&username) {
        return HttpResponse::BadGateway().json(
            serde_json::json!({"status": "fail", "message": "Username already exists"}),
        );
    }

    // create user
    log::info!("Creating user {username}");

    data.db
        .create_user(&user.google_user.id, &user.google_user.email, &username);

    // add token to db
    data.db.add_user_token(&user.google_user.id, &token);

    // Back to homepage
    HttpResponse::Found().header("Location", "/").finish()
}
