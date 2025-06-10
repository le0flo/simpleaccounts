use crate::{database::PgRepository, users::models::User};
use actix_web::{web, http};

#[derive(serde::Deserialize)]
struct RequestBody {
    identifier: String,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    method: String,
    proof: Option<String>,
}

#[actix_web::post("/login/request")]
pub async fn endpoint(psql_pool: web::Data<sqlx::Pool<sqlx::Postgres>>, body: web::Json<RequestBody>) -> impl actix_web::Responder {
    let user = match User::select(&psql_pool, &body.identifier).await {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    let response = ResponseBody {
        method: user.method.to_owned(),
        proof: user.generate_pgp_challenge(),
    };

    return actix_web::HttpResponse::Ok().json(response);
}
