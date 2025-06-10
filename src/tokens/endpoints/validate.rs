use crate::{database::{PgRepository, RedisRepository}, tokens::models::Token};
use actix_web::{web, http};

#[derive(serde::Deserialize)]
struct RequestBody {
    seed: String,
    stamp: String,
}

#[actix_web::post("/validate")]
pub async fn endpoint(psql_pool: web::Data<sqlx::PgPool>, redis_pool: web::Data<r2d2::Pool<redis::Client>>, body: web::Json<RequestBody>) -> impl actix_web::Responder {
    let token = Token::from(&body.seed, &body.stamp);

    if token.validate().is_err() {
        return actix_web::HttpResponse::new(http::StatusCode::NOT_ACCEPTABLE);
    }

    if Token::select(&psql_pool, &body.seed).await.is_err() {
        return actix_web::HttpResponse::new(http::StatusCode::NOT_ACCEPTABLE);
    }

    if Token::put(&redis_pool, &body.seed, 1).is_err() {
        return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    return match token.insert(&psql_pool).await {
        Ok(_) => actix_web::HttpResponse::new(http::StatusCode::OK),
        Err(_) => actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
    };
}
