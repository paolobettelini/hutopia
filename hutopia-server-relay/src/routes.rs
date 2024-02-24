use crate::auth::g_auth::*;
use crate::auth::utils::*;
use crate::*;
use actix_web::cookie::Cookie;
use serde_json::json;
use serde::Serialize;

use actix_web::cookie::time::OffsetDateTime;

/// Redirects the user to the google login page
#[get("/api/login")]
async fn login(_req: HttpRequest, data: web::Data<ServerData>) -> impl Responder {
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
            json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = request_token(code.as_str(), &data).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();

    let access_token = token_response.access_token;
    let id_token = token_response.id_token;

    let google_user = get_google_user(&access_token, &id_token).await;
    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(json!({"status": "fail", "message": message}));
    }

    let google_user = google_user.unwrap();

    // Generate a random session token
    let token = random_token();
    // Give token as a cookies
    let token_cookie = Cookie::build("token", &token).path("/").finish();

    if data.db.user_id_exists(&google_user.id) {
        // user has an account

        let username = data.db.get_user_by_id(&google_user.id).unwrap().username;
        let username_cookie = Cookie::build("username", &username).path("/").finish();

        // add token to db
        data.db.add_user_token(&google_user.id, &token);

        // TODO replace token if expired

        return HttpResponse::Found()
            .cookie(token_cookie)
            .cookie(username_cookie)
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
            .cookie(token_cookie)
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
                json!({"status": "fail", "message": "You are not authenticated"}),
            );
        }
    };

    // TODO: maybe if the user has the "username" token,
    // return to /
    // also in the /api/login if the user has either the token or username cookie

    // check if user is authenticated
    let user = match data.take_unregistered_user(token.clone()) {
        Some(user) => user,
        None => {
            return HttpResponse::BadGateway().json(
                json!({"status": "fail", "message": "You are not authenticated"}),
            );
        }
    };

    // check if username already exists
    if data.db.username_exists(&username) {
        return HttpResponse::BadGateway()
            .json(json!({"status": "fail", "message": "Username already exists"}));
    }

    // create user
    log::info!("Creating user {username}");

    data.db
        .create_user(&user.google_user.id, &user.google_user.email, &username);

    // add token to db
    data.db.add_user_token(&user.google_user.id, &token);

    let username_cookie = Cookie::build("username", &username).path("/").finish();

    // Back to homepage
    HttpResponse::Found()
        .cookie(username_cookie)
        .header("Location", "/")
        .finish()
}

#[derive(Serialize, Debug, Default)]
struct UserData {
    pub logged: bool,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[post("/api/userData")]
async fn user_data(req: HttpRequest, data: web::Data<ServerData>) -> impl Responder {
    // Authenticate
    let user = match authenticate(&req, &data) {
        Some(user) => user,
        None => return not_logged(),
    };

    let user_data = UserData {
        logged: true,
        username: Some(user.username),
        email: Some(user.email),
    };

    let json = serde_json::to_string(&user_data).expect("Failed to serialize");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}

#[get("/api/logout")]
async fn logout(_req: HttpRequest) -> HttpResponse {
    let cookie1 = Cookie::build("username", "")
        .path("/")
        .expires(OffsetDateTime::UNIX_EPOCH)
        .finish();

    let cookie2 = Cookie::build("token", "")
        .path("/")
        .expires(OffsetDateTime::UNIX_EPOCH)
        .finish();

    HttpResponse::Found()
        .header("Location", "/")
        .cookie(cookie1)
        .cookie(cookie2)
        .finish()
}

/// Generate a random token for the user to be sent to a space
#[post("/api/genSpaceAuthToken")]
async fn gen_space_auth_token(req: HttpRequest, data: web::Data<ServerData>) -> impl Responder {
    // Authenticate
    let user = match authenticate(&req, &data) {
        Some(user) => user,
        None => return not_logged(),
    };

    let token = random_token();
    data.add_space_auth_token(user.username.clone(), token.clone());

    log::info!("User {} now has token {}", &user.username, &token);

    let json = json!({
        "token": token,
        "username": user.username, // just for good measure
    });

    HttpResponse::Ok().json(json)
}

/// Sent by a space server to authenticate a user.
#[post("/api/checkSpaceAuthToken/{username}/{token}")]
async fn check_space_auth_token(
    path: web::Path<(String, String)>,
    data: web::Data<ServerData>,
) -> impl Responder {
    let authenticated = data.take_space_auth_tokens(&path.0, &path.1);

    log::info!("User {} has token {}: {authenticated}", &path.0, &path.1);

    let json = json!({
        "authenticated": authenticated
    });

    HttpResponse::Ok().json(json)
}

pub fn not_logged() -> HttpResponse {
    let data = UserData {
        logged: false,
        ..Default::default()
    };
    let json = serde_json::to_string(&data).expect("Failed to serialize");

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}
