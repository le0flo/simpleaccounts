use crate::{database::PgRepository, randoms};

use sequoia_openpgp::parse::Parse;
use sqlx::Row;

pub struct User {
    pub identifier: String,
    pub method: String,
    pub secret: String,
}

impl User {
    pub fn new(method: &String, pubkey: &Option<String>) -> Result<Self, ()> {
        let identifier = randoms::numeric_string(20);

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

        return Ok(User {
            identifier: identifier.to_owned(),
            method: method.to_owned(),
            secret: _secret,
        });
    }

    fn generate_totp_secret(issuer: &String) -> Result<String, ()> {
        let _secret = randoms::alphanumeric_string(64);

        return match totp_rs::TOTP::new(
            totp_rs::Algorithm::SHA1,
            6,
            1,
            30,
            _secret.into_bytes(),
            Some("Simple Accounts".to_string()),
            issuer.clone()
        ) {
            Ok(value) => Ok(value.get_url()),
            Err(_) => Err(()),
        };
    }

    fn validate_pgp_pubkey(pubkey: &Option<String>) -> Result<String, ()> {
        if pubkey.is_none() {
            return Err(());
        }

        let _pubkey_bytes = pubkey.as_ref().unwrap().as_bytes();

        // TODO

        Err(())
    }

    pub fn generate_pgp_challenge(&self) -> Result<Option<String>, ()> {
        if self.method.as_str() == "totp" {
            return Ok(None);
        }

        // TODO + vedere anyhow

        let policy = StandardPolicy::new();
        let pubkey = match sequoia_openpgp::Cert::from_bytes(self.secret.as_bytes()) {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        let mut sink = Vec::new();
        let message = Message::new(&mut sink);

        let message = match Encryptor::for_recipients(message, vec![pubkey]).build() {
            Ok(value) => value,
            Err(_) => return Err(()),
        };

        let mut message = LiteralWriter::new(message).build()?;

        message.write_all(_secret.as_bytes())?;
        message.finalize()?;

        return Ok(Some(String::from_utf8(sink)));
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
