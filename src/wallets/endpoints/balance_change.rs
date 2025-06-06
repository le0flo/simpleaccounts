use crate::{configuration, database::PgRepository, wallets::models::Wallet};
use actix_web::{web, http};

#[derive(serde::Deserialize)]
struct RequestBody {
    admin_key: String,
    identifier: String,
    amount: u32,
    multiplier: i32,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    balance: i32,
}

#[actix_web::put("/balance/change")]
pub async fn endpoint(config: web::Data<configuration::Configuration>, psql_pool: web::Data<sqlx::PgPool>, body: web::Json<RequestBody>) -> impl actix_web::Responder {
    if config.admin.key.eq(&body.admin_key) {
        return actix_web::HttpResponse::new(http::StatusCode::UNAUTHORIZED);
    }

    let mut wallet = match Wallet::select(&psql_pool, &body.identifier).await {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    if wallet.balance_change(body.amount, body.multiplier).is_err() {
        return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    if wallet.update(&psql_pool).await.is_err() {
        return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let response = ResponseBody {
        balance: wallet.balance.clone(),
    };

    return actix_web::HttpResponse::Ok().json(response);
}
