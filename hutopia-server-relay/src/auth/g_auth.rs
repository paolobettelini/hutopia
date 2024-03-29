use crate::*;

use reqwest::{Client, Url};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
    // TODO refresh secret and refresh implementation
}

/// Google login response code
#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: Option<bool>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

/// Users who have signed in for the first time,
/// but have yet to register an account
/// on the register page.
#[derive(Debug)]
pub struct UnregisteredUser {
    pub google_user: GoogleUserResult,
    pub access_token: String,
    pub id_token: String,
}

pub fn gen_login_url(client_id: &str, redirect: &str) -> String {
    const CHOOSE_ACCOUNT: &str = "https://accounts.google.com/o/oauth2/v2/auth/oauthchooseaccount";

    let mut url = Url::parse(CHOOSE_ACCOUNT).unwrap();

    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("scope", "https://www.googleapis.com/auth/userinfo.email")
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", redirect)
        .append_pair("service", "lso")
        .append_pair("o2v", "2");

    url.into_string()
}

pub async fn request_token(
    authorization_code: &str,
    data: &web::Data<ServerData>,
) -> Result<OAuthResponse, Box<dyn Error>> {
    let redirect_url = &data.auth.redirect_url;
    let client_id = &data.auth.client_id;
    let client_secret = &data.auth.client_secret;

    let root_url = "https://oauth2.googleapis.com/token";
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];
    let response = client.post(root_url).form(&params).send().await?;

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
