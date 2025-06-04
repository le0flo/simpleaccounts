use std::{fs, path::Path, process::exit};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub ip: String,
    pub port: u16,
    pub admin: AdminConfiguration,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AdminConfiguration {
    pub key: String,
}

impl Configuration {
    fn new() -> Configuration {
        return Configuration {
            ip: String::from("127.0.0.1"),
            port: 8080,
            admin: AdminConfiguration::new(),
        };
    }

    pub fn load() -> Configuration {
        let file = Path::new("Settings.toml");

        if file.is_file() {
            let dumped = match fs::read_to_string(file) {
                Ok(value) => value,
                Err(_) => exit(-1),
            };

            return match toml::from_str(dumped.as_str()) {
                Ok(value) => value,
                Err(_) => exit(-2),
            };
        } else {
            let config = Configuration::new();
            let config_string = match toml::to_string(&config) {
                Ok(value) => value,
                Err(_) => exit(-3),
            };

            match fs::write(file, config_string) {
                Ok(_) => (),
                Err(_) => exit(-4),
            };

            return config;
        }
    }
}

impl AdminConfiguration {
    fn new() -> AdminConfiguration {
        let _key = uuid::Uuid::new_v4().to_string();

        return AdminConfiguration {
            key: _key,
        };
    }

    pub fn check(&self, to_check: &String) -> Result<(), ()> {
        if self.key.eq(to_check) {
            return Ok(());
        }

        return Err(());
    }
}
