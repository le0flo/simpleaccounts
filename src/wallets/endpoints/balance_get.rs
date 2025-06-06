use crate::{database::{PgRepository, RedisRepository}, sessions::models::UserSession, wallets::models::Wallet};
use actix_web::{web, http};

#[derive(serde::Deserialize)]
struct RequestBody {
    session_id: String,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    balance: i32,
}

#[actix_web::put("/balance/get")]
pub async fn endpoint(psql_pool: web::Data<sqlx::PgPool>, redis_pool: web::Data<r2d2::Pool<redis::Client>>, body: web::Json<RequestBody>) -> impl actix_web::Responder {
    let identifier = match UserSession::get(&redis_pool, &body.session_id) {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::UNAUTHORIZED),
    };

    let wallet = match Wallet::select(&psql_pool, &identifier).await {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = ResponseBody {
        balance: wallet.balance.clone(),
    };

    return actix_web::HttpResponse::Ok().json(response);
}
