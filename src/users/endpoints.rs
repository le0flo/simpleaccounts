use crate::database::postgresql::Repository;
use super::models::{User, BalanceOperation};
use serde::{Deserialize, Serialize};
use actix_web::{http::{header::HeaderName, StatusCode}, web, HttpRequest, HttpResponse, Responder, Scope};
use redis::Commands;

#[derive(Serialize, Deserialize)]
pub struct BalanceChangeRequest {
    pub amount: u32,
    pub operation: BalanceOperation,
}

#[derive(Serialize, Deserialize)]
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
pub async fn new_user(psql_pool: web::Data<sqlx::PgPool>, redis_pool: web::Data<r2d2::Pool<redis::Client>>, request: HttpRequest) -> impl Responder { 
    let mut redis = match redis_pool.get() {
        Ok(value) => value,
        Err(_) => return HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE),
    };
    
    let seed = match request.headers().get(HeaderName::from_static("SA-Token")) {
        Some(value) => value.to_str().unwrap(),
        None => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    let is_solved = match redis.get::<&str, i32>(seed) {
        Ok(value) => value,
        Err(_) => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    if is_solved == 1 {
        if redis.del::<&str, ()>(seed).is_err() {
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
pub async fn delete_user(admin_key: web::Data<uuid::Uuid>, psql_pool: web::Data<sqlx::PgPool>, request: HttpRequest, body: web::Json<UserDeletionRequest>) -> impl Responder { 
    let request_key = match request.headers().get(HeaderName::from_static("SA-AdminKey")) {
        Some(value) => value.to_str().unwrap(),
        None => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    if !admin_key.to_string().eq(request_key) {
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }
    
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
pub async fn balance_change(admin_key: web::Data<uuid::Uuid>, psql_pool: web::Data<sqlx::PgPool>, request: HttpRequest, body: web::Json<BalanceChangeRequest>) -> impl Responder {
    let request_key = match request.headers().get(HeaderName::from_static("SA-AdminKey")) {
        Some(value) => value.to_str().unwrap(),
        None => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    let identifier = match request.headers().get(HeaderName::from_static("SA-User")) {
        Some(value) => String::from(value.to_str().unwrap()),
        None => return HttpResponse::new(StatusCode::BAD_REQUEST),
    };

    if !admin_key.to_string().eq(request_key) {
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }

    let mut user = match User::select(&psql_pool, &identifier).await {
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
