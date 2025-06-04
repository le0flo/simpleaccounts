use crate::database::{PgRepository, RedisRepository};
use super::DEFAULT_BITS;

use rand::{distr::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use redis::Commands;

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub seed: String,
    pub bits: i32,
    pub stamp: String,
}

impl Token {
    pub fn new() -> Self {
        let _seed = rand::rng()
            .sample_iter(Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>();

        let token = Token {
            seed: _seed,
            bits: DEFAULT_BITS,
            stamp: String::from(""),
        };

        return token;
    }

    pub fn validate(&self) -> Result<(), ()> {
        let temp = match hashcash::Token::from_str(&self.stamp) {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        if temp.bits == DEFAULT_BITS as u32 && temp.resource.eq(&self.seed) {
            return match temp.check() {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            };
        }

        return Err(());
    }
}

impl PgRepository<Token> for Token {
    async fn select(db: &sqlx::PgPool, id: &String) -> Result<Token, sqlx::Error> {
        let query = sqlx::query("select bits, stamp from tokens where seed = $1");

        let row = query.bind(id).fetch_one(db).await?;

        let token = Token {
            seed: id.clone(),
            bits: row.try_get::<i32, &str>("bits")?,
            stamp: row.try_get::<String, &str>("stamp")?,
        };

        return Ok(token);
    }

    async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("insert into tokens (seed, bits, stamp) values ($1, $2, $3)");

        query
            .bind(&self.seed)
            .bind(&self.bits)
            .bind(&self.stamp.clone())
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    async fn update(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("update tokens set bits = $2, stamp = $3 where seed = $1");

        query
            .bind(&self.seed)
            .bind(&self.bits)
            .bind(&self.stamp)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    async fn delete(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("delete from tokens where seed = $1");

        query.bind(&self.seed).execute(&mut *transaction).await?;

        transaction.commit().await?;

        return Ok(());
    }
}

impl RedisRepository<i32> for Token {
    fn get(db: &r2d2::Pool<redis::Client>, key: &String) -> Result<i32, ()> {
        let mut conn = match db.get() {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        return match conn.get::<&str, i32>(key.as_str()) {
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

    fn put(db: &r2d2::Pool<redis::Client>, key: &String, value: i32) -> Result<(), ()> {
        let mut conn = match db.get() {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        return match conn.set::<&str, i32, ()>(key.as_str(), value) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        };
    }
}
