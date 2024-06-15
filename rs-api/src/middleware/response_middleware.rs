use actix_service::{Service, Transform};
use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse, ResponseError,
};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::response::{common::BaseResponse, exception::ServerError};

pub struct ResponseMiddleware;

impl<S> Transform<S, ServiceRequest> for ResponseMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = ResponseMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ResponseMiddlewareMiddleware { service })
    }
}

pub struct ResponseMiddlewareMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for ResponseMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            let (req, res) = res.into_parts();
            // let res = match res.error() {
            //     Some(error) => {
            //         let server_error = error.as_error();
            //         let server_error = error.as_error().downcast_ref::<ServerError>().unwrap_or(&ServerError::InternalServerError);
            //         ServiceResponse::new(req, HttpResponse::from_error(server_error.clone()).map_into_boxed_body())
            //     }
            //     None => ServiceResponse::new(req, res.map_into_boxed_body()),
            // };
            let res = match res.error() {
                Some(error) => {
                    // let server_error = error.downcast_ref::<ServerError>().unwrap_or(&ServerError::InternalServerError);
                    if let Some(server_error) = error.as_error::<ServerError>() {
                        // HttpResponse::from_error(server_error.into())
                        // 这里 server_error 的类型是 &ServerError，需要先将引用转换为具体类型的实例，然后再将其放入 Box 中
                        // 常见的做法是克隆 ServerError 实例，如果你不想克隆，可以使用 Arc（原子引用计数）来共享所有权
                        // 方案一，需要在 ServerError 结构体上派生 Clone trait
                        HttpResponse::from_error(actix_web::Error::from(Box::new(
                            server_error.clone(),
                        )
                            as Box<dyn ResponseError>))
                    } else {
                        let error_msg = error.to_string();
                        HttpResponse::InternalServerError().json(BaseResponse::<()>::from_error(
                            &ServerError::InternalServerError(error_msg),
                        ))
                    }
                }
                None => res,
            };

            Ok(ServiceResponse::new(req, res))
        })
    }
}
