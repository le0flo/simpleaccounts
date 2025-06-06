use crate::{database::PgRepository, users::models::User};

use sqlx::Row;

pub struct Wallet {
    pub identifier: String,
    pub balance: i32,
}

impl Wallet {
    pub fn new(user: &User) -> Self {
        return Wallet {
            identifier: user.identifier.clone(),
            balance: super::DEFAULT_BALANCE,
        };
    }

    pub fn balance_change(&mut self, amount: u32, multiplier: i32) -> Result<(), ()> {
        let amount_checks = amount > 0;
        let multiplier_checks = multiplier == -1 || multiplier == 1;
        let balance_checks = (self.balance + (amount as i32 * multiplier)) >= 0;

        if amount_checks && multiplier_checks && balance_checks {
            self.balance += amount as i32 * multiplier;
            return Ok(());
        }

        return Err(());
    }
}

impl PgRepository<Wallet> for Wallet {
    async fn select(db: &sqlx::PgPool, id: &String) -> Result<Wallet, sqlx::Error> {
        let query = sqlx::query("select balance from wallets where identifier = $1");

        let row = query.bind(id).fetch_one(db).await?;

        let wallet = Wallet {
            identifier: id.clone(),
            balance: row.try_get::<i32, &str>("balance")?,
        };

        return Ok(wallet);
    }

    async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("insert into wallets (identifier, balance) values ($1, $2)");

        query
            .bind(&self.identifier)
            .bind(&self.balance)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    async fn update(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("update wallets set balance = $2 where identifier = $1");

        query
            .bind(&self.identifier)
            .bind(&self.balance)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }

    async fn delete(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        let mut transaction = db.begin().await?;

        let query = sqlx::query("delete from wallets where identifier = $1");

        query
            .bind(&self.identifier)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        return Ok(());
    }
}
