use crate::database::postgresql::Repository;
use super::models::Token;
use actix_web::{http::StatusCode, web, HttpResponse, Responder, Scope};
use redis::Commands;

pub fn services() -> Scope {
    web::scope("/token")
        .service(new_token)
        .service(validate_token)
}

#[actix_web::get("/new")]
pub async fn new_token(redis_pool: web::Data<r2d2::Pool<redis::Client>>) -> impl Responder {
    let mut redis = match redis_pool.get() {
        Ok(value) => value,
        Err(_) => return HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE),
    };
    
    let token = Token::new();

    return match redis.set::<&str, i32, ()>(token.seed.as_str(), 0) {
        Ok(_) => HttpResponse::Ok().body(serde_json::to_string(&token).unwrap()),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

#[actix_web::post("/validate")]
pub async fn validate_token(psql_pool: web::Data<sqlx::PgPool>, redis_pool: web::Data<r2d2::Pool<redis::Client>>, body: web::Json<Token>) -> impl Responder {
    let mut redis = match redis_pool.get() {
        Ok(value) => value,
        Err(_) => return HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE),
    };

    if body.validate().is_err() {
        return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
    }

    if Token::select(&psql_pool, &body.seed).await.is_err() {
        return HttpResponse::new(StatusCode::NOT_ACCEPTABLE);
    }

    if redis.set::<&str, i32, ()>(body.seed.as_str(), 1).is_err() {
        return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
    }

    return match body.insert(&psql_pool).await {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };
}
