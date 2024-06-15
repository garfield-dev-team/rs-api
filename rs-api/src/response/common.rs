use actix_web::{http::header::CONTENT_TYPE, HttpResponse, Responder};
use serde::Serialize;

use super::exception::ServerError;

#[derive(Serialize)]
pub struct BaseResponse<T> {
    pub code: u32,
    pub data: Option<T>,
    pub reason: String,
}

impl<T> BaseResponse<T> {
    pub fn from_error(error: &ServerError) -> Self {
        BaseResponse {
            // code: format!("{:?}", error),
            code: error.code(),
            data: None,
            reason: error.to_string(),
        }
    }

    pub fn from_data(data: T) -> Self {
        BaseResponse {
            code: 0,
            data: Some(data),
            reason: "".to_string(),
        }
    }
}

impl<T: Serialize> Responder for BaseResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "application/json"))
            .json(self)
    }
}
