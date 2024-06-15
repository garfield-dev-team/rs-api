use super::constants::ServerError;

#[derive(Debug)]
pub struct BaseResponse<T> {
    pub code: ServerError,
    pub data: Option<T>,
    pub reason: String,
}

impl<T> BaseResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: ServerError::Ok,
            data: Some(data),
            reason: "".to_string(),
        }
    }

    pub fn server_error(reason: String) -> Self {
        Self {
            code: ServerError::InternalServerError,
            data: None,
            reason,
        }
    }

    pub fn bad_request(reason: String) -> Self {
        Self {
            code: ServerError::BadRequest,
            data: None,
            reason,
        }
    }

    pub fn unauthorized(reason: String) -> Self {
        Self {
            code: ServerError::Unauthorized,
            data: None,
            reason,
        }
    }
}

fn test() {
    let a = BaseResponse::ok(1);
    let b = BaseResponse::<()>::server_error("reason".to_string());
    println!("{:?}", a);
}
