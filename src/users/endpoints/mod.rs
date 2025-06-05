pub mod user_new;
pub mod user_delete;
pub mod balance_change;

pub fn services() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(user_new::endpoint)
        .service(user_delete::endpoint)
        .service(balance_change::endpoint)
}
