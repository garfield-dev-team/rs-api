use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use handler::login::{login, register};
use middleware::{
    logging_middleware::LoggingMiddleware, response_middleware::ResponseMiddleware,
    traceid_middleware::AddTraceIdToResponse,
};
use response::{common::BaseResponse, exception::ServerError};
use tracing::Level;
use tracing_actix_web::TracingLogger;
use tracing_subscriber;

mod handler;
mod middleware;
mod model;
mod response;
mod utils;

async fn index() -> Result<HttpResponse, ServerError> {
    Err(ServerError::BadRequest("".to_string())) // 示例错误
}

async fn hello() -> Result<impl Responder, ServerError> {
    Ok(BaseResponse::from_data("Hello, world!".to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化 tracing 订阅者
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    HttpServer::new(|| {
        App::new()
            .wrap(ResponseMiddleware)
            .wrap(AddTraceIdToResponse)
            .wrap(TracingLogger::default())
            .wrap(LoggingMiddleware)
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
            .route("/register", web::get().to(register))
            .route("/login", web::get().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
