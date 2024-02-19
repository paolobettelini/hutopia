use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SpaceConfig {
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
}