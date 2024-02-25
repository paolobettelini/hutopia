use actix_web::HttpRequest;

/// Option<Username>
pub fn auth_user(req: &HttpRequest) -> Option<String> {
    let username = req.cookie("username").unwrap().value().to_string();
    let token = req.cookie("token").unwrap().value().to_string();
    let url = format!("http://127.0.0.1:8080/internal/auth/{username}/{token}");
    let client = reqwest::blocking::Client::new();
    let response = client.post(&url).send().unwrap();
    let json: serde_json::Value = response.json().unwrap();
    let authenticated: bool = json.get("authenticated").and_then(|v| v.as_bool()).unwrap();

    if authenticated {
        Some(username.to_owned())
    } else {
        None
    }
}

// TODO another function for a generic call to the /internal API