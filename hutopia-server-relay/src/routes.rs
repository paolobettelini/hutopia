use crate::*;
use crate::auth::g_auth::*;
use actix_web::*;
use reqwest::{Client, Url};
use std::error::Error;

// https://accounts.google.com/o/oauth2/v2/auth/oauthchooseaccount
// ?client_id=902072777319-fq9amb5e289ldoo4qu5nmoej3bdrp30d.apps.googleusercontent.com
// &state=VWDAQvOwmSAOYEHP1Ja3EFJVMBmufj4L
// &scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email%20https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.profile
// &response_type=code&redirect_uri=https%3A%2F%2Fapi.modrinth.com%2Fv2%2Fauth%2Fcallback
// https://api.modrinth.com/v2/auth/callback
// &service=lso&o2v=2
// &theme=glif
// &flowName=GeneralOAuthFlow
#[get("/api/login")]
async fn login(req: HttpRequest, data: web::Data<ServerData>) -> impl Responder {
    let client_id = &data.auth.client_id;
    let fallback = &data.auth.redirect_url;

    let mut url =
        Url::parse("https://accounts.google.com/o/oauth2/v2/auth/oauthchooseaccount").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("scope", "https://www.googleapis.com/auth/userinfo.email")
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &fallback)
        .append_pair("service", "lso")
        .append_pair("o2v", "2");
    let url = url.into_string();

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
    let google_user = get_google_user(&token_response.access_token, &token_response.id_token).await;
    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let google_user = google_user.unwrap();

    let body = format!("Google Account: {:#?}", google_user);
    HttpResponse::Ok().body(body)
}