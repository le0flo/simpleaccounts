use crate::{database::PgRepository, http_utils, configuration, users::models::User};
use actix_web::{web, http};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BalanceChangeRequest {
    pub amount: u32,
    pub multiplier: i32,
}

#[actix_web::put("/balance/change")]
pub async fn endpoint(config: web::Data<configuration::Configuration>, psql_pool: web::Data<sqlx::PgPool>, request: actix_web::HttpRequest, body: web::Json<BalanceChangeRequest>) -> impl actix_web::Responder {
    let admin_key = match http_utils::get_header_value(&request, "sa-adminkey") {
      Ok(value) => value,
      Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    let user_id = match http_utils::get_header_value(&request, "sa-userid") {
      Ok(value) => value,
      Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    if config.admin.key.eq(&admin_key) {
        return actix_web::HttpResponse::new(http::StatusCode::UNAUTHORIZED);
    }

    let mut user = match User::select(&psql_pool, &user_id).await {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    if user.change_balance(body.amount, body.multiplier).is_err() {
        return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    return match user.update(&psql_pool).await {
        Ok(_) => actix_web::HttpResponse::new(http::StatusCode::OK),
        Err(_) => actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
    };
}
