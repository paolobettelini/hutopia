use actix_web::*;
use std::error::Error;
use crate::*;
use reqwest::{Client, Url};

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
    let client_id = std::env::var("G_AUTH_CLIENT_ID")
        .expect("G_AUTH_CLIENT_ID Env var to be set.");

    let fallback = std::env::var("REDIRECT_URL")
        .expect("REDIRECT_URL Env var to be set.");

    let mut url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth/oauthchooseaccount").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("scope", "https://www.googleapis.com/auth/userinfo.email")
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &fallback)
        .append_pair("service", "lso")
        .append_pair("o2v", "2");
    let url = url.into_string();

    // Redirect the user to the Google login URI
    HttpResponse::Found()
        .header("Location", url)
        .finish()
}

#[get("/api/g_auth")]
async fn login_fallback(query: web::Query<QueryCode>) -> impl Responder {
    let code = &query.code;
    
    if code.is_empty() {
        return HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = request_token(code.as_str()).await;
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

// TEMP

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

/// Google login response code
#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GoogleUserResult {
    pub id: Option<String>,
    pub email: Option<String>,
    pub verified_email: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

pub async fn request_token(
    authorization_code: &str,
) -> Result<OAuthResponse, Box<dyn Error>> {
    let redirect_url = std::env::var("REDIRECT_URL")
        .expect("REDIRECT_URL Env var to be set.");
    let client_secret = std::env::var("G_AUTH_SECRET")
        .expect("G_AUTH_SECRET Env var to be set.");
    let client_id = std::env::var("G_AUTH_CLIENT_ID")
        .expect("G_AUTH_CLIENT_ID Env var to be set.");

    let root_url = "https://oauth2.googleapis.com/token";
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];
    let response = client
        .post(root_url)
        .form(&params)
        .send().await?;

    if response.status().is_success() {
        let oauth_response = response.json::<OAuthResponse>().await?;
        Ok(oauth_response)
    } else {
        log::error!("{response:?}");
        let message = "An error occurred while trying to retrieve access token.";
        Err(From::from(message))
    }
}

pub async fn get_google_user(
    access_token: &str,
    id_token: &str,
) -> Result<GoogleUserResult, Box<dyn Error>> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client.get(url).bearer_auth(id_token).send().await?;

    if response.status().is_success() {
        let user_info = response.json::<GoogleUserResult>().await?;
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}