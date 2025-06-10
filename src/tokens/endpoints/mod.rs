mod new;
mod validate;

pub fn services() -> actix_web::Scope {
    actix_web::web::scope("/tokens")
        .service(new::endpoint)
        .service(validate::endpoint)
}
