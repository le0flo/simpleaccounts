use crate::{database::RedisRepository, sessions::models::UserSession};
use actix_web::{web, http};

#[derive(serde::Deserialize)]
struct RequestBody {
    session_id: String,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    user_id: String,
}

#[actix_web::get("/validate")]
pub async fn endpoint(redis_pool: web::Data<r2d2::Pool<redis::Client>>, body: web::Json<RequestBody>) -> impl actix_web::Responder {
    let user_id = match UserSession::get(&redis_pool, &body.session_id) {
      Ok(value) => value,
      Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    let response = ResponseBody {
        user_id: user_id.to_owned(),
    };

    return actix_web::HttpResponse::Ok().json(response);
}
