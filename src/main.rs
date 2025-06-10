use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod configuration;
mod database;
mod randoms;

mod sessions;
mod tokens;
mod users;

mod wallets;

#[actix_web::get("/")]
async fn healthcheck() -> impl Responder {
    return HttpResponse::Ok().body("Online!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = configuration::Configuration::load();
    let psql_pool = database::connect_psql("SIMPLEACCOUNTS_PSQL_URI").await;
    let redis_pool = database::connect_redis("SIMPLEACCOUNTS_REDIS_URI").await;

    println!("Current configuration:\n---{}---", config);

    let ip = config.ip.clone();
    let port = config.port.clone();

    return HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.to_owned()))
            .app_data(web::Data::new(psql_pool.to_owned()))
            .app_data(web::Data::new(redis_pool.to_owned()))
            .service(healthcheck)
            .service(sessions::endpoints::services())
            .service(tokens::endpoints::services())
            .service(users::endpoints::services())
            .service(wallets::endpoints::services())
    })
    .bind((ip, port))?
    .workers(2)
    .run()
    .await;
}
