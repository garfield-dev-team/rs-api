use actix_web::{web, Responder};

use crate::{
    model::login::User,
    response::{common::BaseResponse, exception::ServerError},
    utils::crypto::{hash_password, verify_password},
};

pub async fn register(user: web::Json<User>) -> Result<impl Responder, ServerError> {
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

pub async fn login(user: web::Json<User>) -> Result<impl Responder, ServerError> {
    // 在实际应用中，你应该从数据库中检索存储的哈希密码
    let stored_hashed_password = "$2b$12$eXAmpleHashedPassword..."; // 示例哈希密码

    let valid = verify_password(&user.password, &stored_hashed_password).unwrap_or(false);
    if valid {
      Ok(BaseResponse::from_data(format!("User {} logged in successfully!",user.username)))
    } else {
      Err(ServerError::Unauthorized("Invalid username or password".to_string()))
    }
}
