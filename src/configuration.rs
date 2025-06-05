use std::{process, fs, path::Path};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    pub ip: String,
    pub port: u16,
    pub admin: AdminConfiguration,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct AdminConfiguration {
    pub key: String,
}

impl Configuration {
    fn new() -> Configuration {
        return Configuration {
            ip: String::from("0.0.0.0"),
            port: 8080,
            admin: AdminConfiguration::new(),
        };
    }

    pub fn load() -> Configuration {
        let file = Path::new("Settings.toml");

        if file.is_file() {
            let dumped = match fs::read_to_string(file) {
                Ok(value) => value,
                Err(_) => process::exit(0x01),
            };

            return match toml::from_str(dumped.as_str()) {
                Ok(value) => value,
                Err(_) => process::exit(0x02),
            };
        } else {
            let config = Configuration::new();
            let config_string = match toml::to_string(&config) {
                Ok(value) => value,
                Err(_) => process::exit(0x11),
            };

            match fs::write(file, config_string) {
                Ok(_) => (),
                Err(_) => process::exit(0x12),
            };

            return config;
        }
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(formatter, "\n- ip: {}\n- port: {}\n- admin: {}\n", self.ip, self.port, self.admin);
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

impl std::fmt::Display for AdminConfiguration {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(formatter, "\n\t- key: {}\n", self.key);
    }
}
