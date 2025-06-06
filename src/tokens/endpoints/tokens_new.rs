use crate::{database::RedisRepository, tokens::models::Token};
use actix_web::{web, http};

#[actix_web::get("/new")]
pub async fn endpoint(redis_pool: web::Data<r2d2::Pool<redis::Client>>) -> impl actix_web::Responder {
    let token = Token::new();

    return match Token::put(&redis_pool, &token.seed, 0) {
        Ok(_) => actix_web::HttpResponse::Ok().json(token),
        Err(_) => actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
    };
}
