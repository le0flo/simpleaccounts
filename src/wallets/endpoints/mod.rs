mod balance_get;
mod balance_change;

pub fn services() -> actix_web::Scope {
    return actix_web::web::scope("/wallets")
        .service(balance_get::endpoint)
        .service(balance_change::endpoint);
}
