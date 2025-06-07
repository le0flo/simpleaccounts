use crate::{configuration, database::PgRepository, users::models::User, wallets::models::Wallet};
use actix_web::{web, http};

#[derive(serde::Deserialize)]
struct RequestBody {
    admin_key: String,
    identifier: String,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    identifier: String,
}

#[actix_web::delete("/delete")]
pub async fn endpoint(config: web::Data<configuration::Configuration>, psql_pool: web::Data<sqlx::PgPool>, body: web::Json<RequestBody>) -> impl actix_web::Responder {
    match config.admin.check(&body.admin_key) {
        Ok(_) => (),
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::UNAUTHORIZED),
    };

    let user = match User::select(&psql_pool, &body.identifier).await {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    let wallet = match Wallet::select(&psql_pool, &body.identifier).await {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    let _wallet_delete = wallet.delete(&psql_pool).await.is_err();
    let _user_delete = user.delete(&psql_pool).await.is_err();

    if _wallet_delete || _user_delete {
        return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let response = ResponseBody {
        identifier: body.identifier.to_owned(),
    };

    return actix_web::HttpResponse::Ok().json(response);
}
