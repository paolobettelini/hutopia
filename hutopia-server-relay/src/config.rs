use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RelayConfig {
    pub server: Server,
    pub env: Env,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Env {
    #[serde(rename(deserialize = "db-connection"))]
    pub db_connection: String,
    #[serde(rename(deserialize = "g-auth-client-id"))]
    pub g_auth_client_id: String,
    #[serde(rename(deserialize = "g-auth-secret"))]
    pub g_auth_secret: String,
    #[serde(rename(deserialize = "redirect-url"))]
    pub redirect_url: String,
}