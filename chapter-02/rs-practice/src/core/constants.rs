use std::error::Error;
use std::fmt;

// 领券提示
pub static RECEIVE_COUPON: &'static str = "receive coupon";
pub static ADD_CART_COUNT: i32 = 100;

#[derive(Debug)]
pub enum ServerError {
    Ok,
    InternalServerError,
    BadRequest,
    NotFound,
    Unauthorized,
    Forbidden,
}

// 实现Display trait
// Display trait 用于将错误码转换为用户可读的字符串。这里我们为每种错误类型定义了对应的错误信息。
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Ok => write!(f, "200 OK"),
            ServerError::InternalServerError => write!(f, "500 Internal Server Error"),
            ServerError::BadRequest => write!(f, "400 Bad Request"),
            ServerError::NotFound => write!(f, "404 Not Found"),
            ServerError::Unauthorized => write!(f, "401 Unauthorized"),
            ServerError::Forbidden => write!(f, "403 Forbidden"),
        }
    }
}

// 实现Error trait
// Error trait 是一个标记 trait，它没有任何方法需要实现，但它表明这个类型可以用作错误类型。
impl Error for ServerError {}
