mod tokens_new;
mod tokens_validate;

pub fn services() -> actix_web::Scope {
    actix_web::web::scope("/tokens")
        .service(tokens_new::endpoint)
        .service(tokens_validate::endpoint)
}
