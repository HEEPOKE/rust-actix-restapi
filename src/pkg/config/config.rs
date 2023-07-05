use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Result<Self, env::VarError> {
        dotenv().ok();

        let host = env::var("HOST")?;
        let port = env::var("PORT")?.parse::<u16>()?;
        let database_url = env::var("DATABASE_URL")?;

        Ok(Config {
            host,
            port,
            database_url,
        })
    }
}

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| Config::load().expect("Failed to load configuration"));
