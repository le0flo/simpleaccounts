use crate::{configuration::Configuration, database::{PgRepository, RedisRepository}, http, tokens::models::Token};
use super::models::{BalanceOperation, User};

use serde::{Deserialize, Serialize};
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder, Scope};

#[derive(Deserialize, Serialize)]
pub struct BalanceChangeRequest {
    pub amount: u32,
    pub operation: BalanceOperation,
}

#[derive(Deserialize, Serialize)]
pub struct UserDeletionRequest {
    pub identifier: String,
}

pub fn services() -> Scope {
    web::scope("/user")
        .service(new_user)
        .service(delete_user)
        .service(balance_change)
}

#[actix_web::get("/new")]
pub async fn new_user(psql_pool: web::Data<sqlx::Pool<sqlx::Postgres>>, redis_pool: web::Data<r2d2::Pool<redis::Client>>, request: HttpRequest) -> impl Responder {
    let seed = match http::get_header_value(&request, "sa-token") {
      Ok(value) => value,
      Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    let is_solved = match Token::get(&redis_pool, &seed) {
        Ok(value) => value,
        Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    if is_solved == 1 {
        if Token::del(&redis_pool, &seed).is_err() {
            return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
        }

        let user = User::new();

        return match user.insert(&psql_pool).await {
            Ok(_) => HttpResponse::Ok().body(serde_json::to_string(&user).unwrap()),
            Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        };
    }

    return HttpResponse::new(StatusCode::BAD_REQUEST);
}

#[actix_web::delete("/delete")]
pub async fn delete_user(config: web::Data<Configuration>, psql_pool: web::Data<sqlx::PgPool>, request: HttpRequest, body: web::Json<UserDeletionRequest>) -> impl Responder {
    let admin_key = match http::get_header_value(&request, "sa-adminkey") {
      Ok(value) => value,
      Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    match config.admin.check(&admin_key) {
        Ok(_) => (),
        Err(_) => return HttpResponse::new(StatusCode::UNAUTHORIZED),
    };

    let user = match User::select(&psql_pool, &body.identifier).await {
        Ok(value) => value,
        Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    return match user.delete(&psql_pool).await {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

#[actix_web::put("/balance/change")]
pub async fn balance_change(config: web::Data<Configuration>, psql_pool: web::Data<sqlx::PgPool>, request: HttpRequest, body: web::Json<BalanceChangeRequest>) -> impl Responder {
    let admin_key = match http::get_header_value(&request, "sa-adminkey") {
      Ok(value) => value,
      Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    let user_id = match http::get_header_value(&request, "sa-userid") {
      Ok(value) => value,
      Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    if config.admin.key.eq(&admin_key) {
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }

    let mut user = match User::select(&psql_pool, &user_id).await {
        Ok(value) => value,
        Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    if user.change_balance(body.amount, body.operation).is_err() {
        return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR);
    }

    return match user.update(&psql_pool).await {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };
}
