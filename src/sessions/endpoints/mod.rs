mod login_request;
mod login_verify;

pub fn services() -> actix_web::Scope {
    return actix_web::web::scope("/sessions")
        .service(login_request::endpoint)
        .service(login_verify::endpoint);
}
