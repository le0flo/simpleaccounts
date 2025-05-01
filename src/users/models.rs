use crate::database::postgresql::Repository;
use super::DEFAULT_BALANCE;
use rand::{distr::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool, Row};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub identifier: String,
    pub balance: i32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum BalanceOperation {
    INCREMENT,
    DECREMENT,
}

impl User {
    pub fn new() -> Self {
        let _identifier = rand::rng()
            .sample_iter(Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>();

        let user = User {
            identifier: _identifier,
            balance: DEFAULT_BALANCE,
        };

        return user;
    }

    pub fn change_balance(&mut self, amount: u32, operation: BalanceOperation) -> Result<(), ()> {
        let multiplier = match operation {
            BalanceOperation::INCREMENT => 1,
            BalanceOperation::DECREMENT => -1,
        };

        if amount > 0 && self.balance >= (amount as i32) {
            self.balance += (amount as i32) * multiplier;
            return Ok(());
        }

        return Err(());
    }
}

impl Repository<User> for User {
    async fn select(db: &PgPool, id: &String) -> Result<User, Error> {
        let query = sqlx::query("select balance from users where identifier = $1");

        let row = query.bind(id).fetch_one(db).await?;

        let user = User {
            identifier: id.clone(),
            balance: row.try_get::<i32, &str>("balance")?,
        };

        return Ok(user);
    }

    async fn insert(&self, db: &PgPool) -> Result<(), Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("insert into users (identifier, balance) values ($1, $2)");

        query
            .bind(&self.identifier)
            .bind(&self.balance)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    async fn update(&self, db: &PgPool) -> Result<(), Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("update users set balance = $2 where identifier = $1");

        query
            .bind(&self.identifier)
            .bind(&self.balance)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    async fn delete(&self, db: &PgPool) -> Result<(), Error> {
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
