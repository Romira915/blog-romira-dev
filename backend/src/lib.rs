use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub postgres_db: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: u32,
    pub database_url: String,
}

impl Config {
    pub fn database_url_as_ref(&self) -> &str {
        self.database_url.as_str()
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv::dotenv().unwrap();
    envy::from_env().unwrap()
});
