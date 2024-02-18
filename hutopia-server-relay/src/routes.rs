use crate::*;
use crate::auth::g_auth::*;
use actix_web::*;
use reqwest::{Client, Url};
use std::error::Error;

#[get("/api/login")]
async fn login(req: HttpRequest, data: web::Data<ServerData>) -> impl Responder {
    let client_id = &data.auth.client_id;
    let redirect = &data.auth.redirect_url;

    let url = gen_login_url(&client_id, &redirect);

    // Redirect the user to the Google login URI
    HttpResponse::Found().header("Location", url).finish()
}

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

    let token_response = request_token(code.as_str(), data).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();

    let access_token = token_response.access_token;
    let id_token = token_response.id_token;

    log::warn!("Access token {access_token}");
    log::warn!("ID token {id_token}");

    let google_user = get_google_user(&access_token, &id_token).await;
    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let google_user = google_user.unwrap();

    let body = format!("Google Account: {:#?}", google_user);
    HttpResponse::Ok().body(body)
}