use crate::{database::RedisRepository, users::models::User};

use redis::Commands;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
}

impl Session {
    pub fn new(user: &User) -> Self {
        return Session {
            session_id: uuid::Uuid::new_v4().to_string(),
            user_id: user.identifier.clone(),
        };
    }
}

impl RedisRepository<String> for Session {
    fn get(db: &r2d2::Pool<redis::Client>, key: &String) -> Result<String, ()> {
        let mut conn = match db.get() {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        return match conn.get::<&str, String>(key.as_str()) {
            Ok(value) => Ok(value),
            Err(_) => Err(()),
        };
    }

    fn del(db: &r2d2::Pool<redis::Client>, key: &String) -> Result<(), ()> {
        let mut conn = match db.get() {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        return match conn.del::<&str, ()>(key.as_str()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        };
    }

    fn put(db: &r2d2::Pool<redis::Client>, key: &String, value: String) -> Result<(), ()> {
        let mut conn = match db.get() {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        return match conn.set::<&str, String, ()>(key.as_str(), value) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        };
    }
}
