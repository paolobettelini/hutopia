use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SpaceConfig {
    pub server: Server,
    pub env: Env,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub relay: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Env {
    #[serde(rename(deserialize = "db-connection"))]
    pub db_connection: String,
}