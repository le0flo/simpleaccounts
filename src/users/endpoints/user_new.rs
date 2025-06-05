use crate::{database::PgRepository, database::RedisRepository, http_utils, tokens::models::Token, users::models::User};
use actix_web::{web, http};

#[actix_web::get("/new")]
pub async fn endpoint(psql_pool: web::Data<sqlx::Pool<sqlx::Postgres>>, redis_pool: web::Data<r2d2::Pool<redis::Client>>, request: actix_web::HttpRequest) -> impl actix_web::Responder {
    let seed = match http_utils::get_header_value(&request, "sa-token") {
      Ok(value) => value,
      Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    let is_solved = match Token::get(&redis_pool, &seed) {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    if is_solved == 1 {
        if Token::del(&redis_pool, &seed).is_err() {
            return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR);
        }

        let user = User::new();

        return match user.insert(&psql_pool).await {
            Ok(_) => actix_web::HttpResponse::Ok().body(serde_json::to_string(&user).unwrap()),
            Err(_) => actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        };
    }

    return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST);
}
