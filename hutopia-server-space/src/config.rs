use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SpaceConfig {
    pub server: Server,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub address: String,
    pub port: u16,
}
