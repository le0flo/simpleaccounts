use crate::{database::{PgRepository, RedisRepository}, sessions::models::Session, tokens::models::Token, users::models::User, wallets::models::Wallet};
use actix_web::{web, http};

#[derive(serde::Deserialize)]
struct RequestBody {
    seed: String,
    method: String,
    secret: Option<String>,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    session_id: String,
    identifier: String,
    secret: String,
}

#[actix_web::post("/new")]
pub async fn endpoint(psql_pool: web::Data<sqlx::Pool<sqlx::Postgres>>, redis_pool: web::Data<r2d2::Pool<redis::Client>>, body: web::Json<RequestBody>) -> impl actix_web::Responder {
    let is_solved = match Token::get(&redis_pool, &body.seed) {
        Ok(value) => value,
        Err(_) => return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST),
    };

    if is_solved == 1 {
        if Token::del(&redis_pool, &body.seed).is_err() {
            return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR);
        }

        let user = match User::new(&body.method, &body.secret) {
            Ok(value) => value,
            Err(_) => return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        };

        let wallet = Wallet::new(&user);
        let session = Session::new(&user);

        let _user_insert = user.insert(&psql_pool).await.is_err();
        let _wallet_insert = wallet.insert(&psql_pool).await.is_err();
        let _session_put = Session::put(&redis_pool, &session.session_id, session.user_id).is_err();

        if _user_insert || _wallet_insert || _session_put {
            return actix_web::HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR);
        }

        let response = ResponseBody {
            session_id: session.session_id.to_owned(),
            identifier: user.identifier.to_owned(),
            secret: user.secret.to_owned(),
        };

        return actix_web::HttpResponse::Ok().json(response);
    }

    return actix_web::HttpResponse::new(http::StatusCode::BAD_REQUEST);
}
