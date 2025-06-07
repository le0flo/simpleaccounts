use crate::database::PgRepository;

use rand::{distr::Alphanumeric, Rng};
use pgp::composed::Deserializable;
use sqlx::Row;

pub struct User {
    pub identifier: String,
    pub method: String,
    pub secret: String,
}

impl User {
    pub fn new(method: &String, pubkey: &Option<String>) -> Result<Self, ()> {
        let identifier = rand::rng()
            .sample_iter(Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>();

        let _secret = match method.as_str() {
            "totp" => match Self::generate_totp_secret(&identifier) {
                Ok(value) => value,
                Err(_) => return Err(()),
            },

            "pgp" => match Self::validate_pgp_pubkey(pubkey) {
                Ok(value) => value,
                Err(_) => return Err(()),
            },

            _ => return Err(()),
        };

        let user = User {
            identifier: identifier.clone(),
            method: method.clone(),
            secret: _secret,
        };

        return Ok(user);
    }

    fn generate_totp_secret(issuer: &String) -> Result<String, ()> {
        let _secret = rand::rng()
            .sample_iter(Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>();

        let totp = match totp_rs::TOTP::new(
            totp_rs::Algorithm::SHA1,
            6,
            1,
            30,
            _secret.into_bytes(),
            Some("Simple Accounts".to_string()),
            issuer.clone()
        ) {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        return Ok(totp.get_url());
    }

    fn validate_pgp_pubkey(pubkey: &Option<String>) -> Result<String, ()> {
        if pubkey.is_some() {
            let _pubkey_string = pubkey.as_ref().unwrap().as_str();

            return match pgp::composed::SignedPublicKey::from_string(_pubkey_string) {
                Ok(_) => Ok(pubkey.as_ref().unwrap().clone()),
                Err(_) => Err(()),
            };
        }

        return Err(());
    }
}

impl PgRepository<User> for User {
    async fn select(db: &sqlx::PgPool, id: &String) -> Result<User, sqlx::Error> {
        let query = sqlx::query("select method, secret from users where identifier = $1");

        let row = query.bind(id).fetch_one(db).await?;

        let user = User {
            identifier: id.clone(),
            method: row.try_get::<String, &str>("method")?,
            secret: row.try_get::<String, &str>("secret")?,
        };

        return Ok(user);
    }

    async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("insert into users (identifier, method, secret) values ($1, $2, $3)");

        query
            .bind(&self.identifier)
            .bind(&self.method)
            .bind(&self.secret)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    async fn update(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("update users set method = $2, secret = $3 where identifier = $1");

        query
            .bind(&self.identifier)
            .bind(&self.method)
            .bind(&self.secret)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    async fn delete(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("delete from users where identifier = $1");

        query
            .bind(&self.identifier)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }
}
