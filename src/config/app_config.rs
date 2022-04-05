use figment::providers::{Toml, Format};
use rocket::figment::Figment;
use rocket::serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppConfig {
    password_salt: String,
    secret_key: String,
}

impl AppConfig {
    pub fn config() -> Self {
        let figment = rocket::Config::figment();
        let profile = figment.profile();
        if profile == "debug" {
            Self::debug_config()
        } else {
            Self::release_config()
        }
    }
    
    fn debug_config() -> Self {
        Figment::new()
            .merge(("password_salt", "rjP6SzSVu1Vi6Q=="))
            .merge(("secret_key", "7dKQPIqBpZuQ/EGmebZByCvsrBNq4DW2TWvL0v4/pPo="))
            .extract::<AppConfig>().expect("Couldn't load configuration")
    }

    fn release_config() -> Self {
        let secret_key = fs::read_to_string("secret_key").expect("Couldn't find a secret_key file");
        Figment::new()
            .merge(("secret_key", secret_key))
            .merge(Toml::file("AppConfig.toml"))
            .extract::<AppConfig>().expect("Couldn't load configuration")
    }

    pub fn get_password_salt(&self) -> &[u8] {
        self.password_salt.as_bytes()
    }
}

