use rand::seq::SliceRandom;
use rand::thread_rng;
use actix_web::HttpResponse;
use hutopia_database_relay::models::User;
use crate::ServerData;
use actix_web::HttpRequest;
use actix_web::web;

/// Returns (username, token) if the user is authenticated.
pub fn authenticate(req: &HttpRequest, data: &web::Data<ServerData>) -> Option<User> {
    // Check is token cookie is set
    let token = match req.cookie("token") {
        Some(token_cookie) => token_cookie.value().to_string(),
        None => return None,
    };

    // Check is username cookie is set
    let username = match req.cookie("username") {
        Some(username_cookie) => username_cookie.value().to_string(),
        None => return None,
    };

    // Get user from database
    let user = data.db.get_user_by_username(&username)?;

    // Check session token
    if !data.db.user_has_token(&user.id, &token) {
        return None;
    }

    // TODO: check expire_date

    Some(user)
}

pub fn random_session_token() -> String {
    let length = 64;
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut rng = thread_rng();
    let token: String = (0..length)
        .map(|_| *alphabet.choose(&mut rng).unwrap())
        .collect();
    token
}