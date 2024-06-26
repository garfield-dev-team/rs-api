use actix_web::{web, Responder};
use anyhow::{Context, Result};

use crate::{
    model::login::UserRequest,
    response::{common::BaseResponse, exception::ServerError},
    utils::crypto::{hash_password, verify_password},
};

#[actix_web::post("/register")]
pub async fn register(user: web::Json<UserRequest>) -> Result<impl Responder, ServerError> {
    if user.username.is_empty() {
        return Err(ServerError::BadRequest(
            "Username cannot be empty".to_string(),
        ));
    }
    if user.password.is_empty() {
        return Err(ServerError::BadRequest(
            "Password cannot be empty".to_string(),
        ));
    }
    match hash_password(&user.password) {
        Ok(hashed_password) => {
            // 在实际应用中，你应该将用户名和哈希密码存储在数据库中
            Ok(BaseResponse::from_data(format!(
                "User {} registered successfully!",
                user.username
            )))
        }
        Err(_) => Err(ServerError::BadRequest(format!(
            "Failed to hash password for user {}",
            user.username
        ))),
    }
}

#[actix_web::post("/login")]
pub async fn login(user: web::Json<UserRequest>) -> Result<impl Responder, ServerError> {
    // 在实际应用中，你应该从数据库中检索存储的哈希密码
    let stored_hashed_password = "$2b$12$eXAmpleHashedPassword..."; // 示例哈希密码

    let valid = verify_password(&user.password, &stored_hashed_password).unwrap_or(false);
    if valid {
        Ok(BaseResponse::from_data(format!(
            "User {} logged in successfully!",
            user.username
        )))
    } else {
        Err(ServerError::Unauthorized(
            "Invalid username or password".to_string(),
        ))
    }
}
