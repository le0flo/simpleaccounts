#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserAuthentication {
    pub identifier: String,
    pub method: String,
    pub secret: String,
}
