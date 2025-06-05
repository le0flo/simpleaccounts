#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserSession {
    pub session_id: String,
    pub user_id: String,
}
