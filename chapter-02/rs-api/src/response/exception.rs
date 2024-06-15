use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::error::Error as StdError;
use std::fmt;

use super::common::BaseResponse;

#[derive(Debug, Clone, Serialize)]
pub enum ServerError {
    BadRequest(String),
    Unauthorized(String),
    InternalServerError(String),
    // 其他错误类型
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::BadRequest(ref msg) => write!(f, "Bad Request: {}", msg),
            ServerError::Unauthorized(ref msg) => write!(f, "Unauthorized: {}", msg),
            ServerError::InternalServerError(ref msg) => {
                write!(f, "Internal Server Error: {}", msg)
            } // 其他错误类型
        }
    }
}

impl ServerError {
    pub fn code(&self) -> u32 {
        match self {
            ServerError::BadRequest(_) => 10001,
            ServerError::Unauthorized(_) => 10002,
            ServerError::InternalServerError(_) => 10003,
            // 其他错误类型的处理
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServerError::BadRequest(_) => {
                HttpResponse::BadRequest().json(BaseResponse::<()>::from_error(self))
            }
            ServerError::Unauthorized(_) => {
                HttpResponse::Unauthorized().json(BaseResponse::<()>::from_error(self))
            }
            ServerError::InternalServerError(_) => {
                HttpResponse::InternalServerError().json(BaseResponse::<()>::from_error(self))
            } // 其他错误类型
        }
    }
}

impl StdError for ServerError {}
