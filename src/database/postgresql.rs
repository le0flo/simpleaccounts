pub trait Repository<T> {
    async fn select(db: &sqlx::PgPool, id: &String) -> Result<T, sqlx::Error>;
    async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error>;
    async fn update(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error>;
    async fn delete(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error>;
}
