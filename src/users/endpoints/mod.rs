mod new;
mod delete;

pub fn services() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(new::endpoint)
        .service(delete::endpoint)
}
