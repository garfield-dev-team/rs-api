use super::constants::ServerError;

pub fn perform_request() -> Result<(), ServerError> {
    // 模拟一个错误
    Err(ServerError::BadRequest)
}
