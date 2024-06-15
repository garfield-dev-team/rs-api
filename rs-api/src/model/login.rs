use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}
