mod sessions_validate;

pub fn services() -> actix_web::Scope {
    actix_web::web::scope("/sessions")
        .service(sessions_validate::endpoint)
}
