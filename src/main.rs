use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::Client;
use sqlx::Postgres;
use uuid::Uuid;
use std::{fs, path::Path, process::exit, str::FromStr};

mod database;
mod tokens;
mod users;

fn generate_admin_key() -> Uuid {
    let file = Path::new(".admin");
    let mut key = uuid::Uuid::new_v4();

    if file.is_file() {
        key = match fs::read_to_string(file) {
            Ok(value) => uuid::Uuid::from_str(value.as_str()).expect("Invalid admin_key format."),
            Err(_) => exit(-1),
        };
    } else {
        match fs::write(file, key.to_string()) {
            Ok(_) => (),
            Err(_) => exit(-2),
        };
    }

    return key;
}

async fn connect_psql(env: &str) -> sqlx::Pool<Postgres> {
    let uri = std::env::var(env).expect("No PSQL_URI variable found.");

    let pool = sqlx::postgres::PgPool::connect(uri.as_str())
        .await
        .expect("The PSQL database is not available.");

    return pool;
}

async fn connect_redis(env: &str) -> r2d2::Pool<Client> {
    let uri = std::env::var(env).expect("No REDIS_URI variable found.");

    let client = redis::Client::open(uri.as_str()).expect("The Redis instance is not available.");

    let pool = r2d2::Pool::builder()
        .build(client)
        .expect("Could not create the Redis pool.");

    return pool;
}

#[actix_web::get("/")]
async fn healthcheck() -> impl Responder {
    return HttpResponse::Ok().body("Online!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 8080;

    let admin_key = generate_admin_key();
    let psql_pool = connect_psql("PSQL_URI").await;
    let redis_pool = connect_redis("REDIS_URI").await;

    println!("Serving on {}:{}", ip, port); // TODO use an actual logger

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(admin_key.to_owned()))
            .app_data(web::Data::new(psql_pool.to_owned()))
            .app_data(web::Data::new(redis_pool.to_owned()))
            .service(healthcheck)
            .service(tokens::endpoints::services())
            .service(users::endpoints::services())
    })
    .bind((ip, port))?
    .workers(2)
    .run()
    .await
}