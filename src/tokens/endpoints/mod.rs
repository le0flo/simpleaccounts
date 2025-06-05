pub mod token_new;
pub mod token_validate;

pub fn services() -> actix_web::Scope {
    actix_web::web::scope("/token")
        .service(token_new::endpoint)
        .service(token_validate::endpoint)
}
