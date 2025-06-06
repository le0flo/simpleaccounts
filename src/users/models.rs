use crate::database::PgRepository;

use rand::{distr::Alphanumeric, Rng};
use sqlx::Row;

pub struct User {
    pub identifier: String,
    pub method: String,
    pub secret: String,
}

impl User {
    pub fn new(method: &String, secret: &Option<String>) -> Self {
        let _identifier = rand::rng()
            .sample_iter(Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>();

        // TODO creare i segreti d'autenticazione e verificarli
        let user = User {
            identifier: _identifier,
            method: method.clone(),
            secret: secret.clone().unwrap(),
        };

        return user;
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
