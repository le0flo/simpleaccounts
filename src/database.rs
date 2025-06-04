pub async fn connect_psql(env: &str) -> sqlx::PgPool {
    let uri = std::env::var(env).expect("No PSQL_URI variable found.");

    let pool = sqlx::PgPool::connect(uri.as_str())
        .await
        .expect("The PSQL database is not available.");

    return pool;
}

pub async fn connect_redis(env: &str) -> r2d2::Pool<redis::Client> {
    let uri = std::env::var(env).expect("No REDIS_URI variable found.");

    let client = redis::Client::open(uri.as_str()).expect("The Redis instance is not available.");

    let pool = r2d2::Pool::builder()
        .build(client)
        .expect("Could not create the Redis pool.");

    return pool;
}

pub trait PgRepository<T> {
    async fn select(db: &sqlx::PgPool, id: &String) -> Result<T, sqlx::Error>;
    async fn insert(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error>;
    async fn update(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error>;
    async fn delete(&self, db: &sqlx::PgPool) -> Result<(), sqlx::Error>;
}

pub trait RedisRepository<T> {
    fn get(db: &r2d2::Pool<redis::Client>, key: &String) -> Result<T, ()>;
    fn del(db: &r2d2::Pool<redis::Client>, key: &String) -> Result<(), ()>;
    fn put(db: &r2d2::Pool<redis::Client>, key: &String, value: T) -> Result<(), ()>;
}
