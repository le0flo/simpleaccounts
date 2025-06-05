use crate::{database::PgRepository, http_utils, configuration, users::models::User};
use actix_web::{web, http};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserDeletionRequest {
    pub identifier: String,
}

#[actix_web::delete("/delete")]
pub async fn endpoint(config: web::Data<configuration::Configuration>, psql_pool: web::Data<sqlx::PgPool>, request: actix_web::HttpRequest, body: web::Json<UserDeletionRequest>) -> impl actix_web::Responder {
    let admin_key = match http_utils::get_header_value(&request, "sa-adminkey") {
      Ok(value) => value,
      Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    match config.admin.check(&admin_key) {
        Ok(_) => (),
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::UNAUTHORIZED),
    };

    let user = match User::select(&psql_pool, &body.identifier).await {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    return match user.delete(&psql_pool).await {
        Ok(_) => actix_web::HttpResponse::new(http::StatusCode::OK),
        Err(_) => actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
    };
}
