mod users_new;
mod users_delete;

pub fn services() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(users_new::endpoint)
        .service(users_delete::endpoint)
}
