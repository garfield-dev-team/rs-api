use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures_util::future::{ok, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
// use std::future::{ready, Ready as StdReady};

pub struct LoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddlewareMiddleware { service })
    }
}

pub struct LoggingMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures_util::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 记录请求入参
        let path = req.path().to_string();
        let method = req.method().to_string();
        let query_string = req.query_string().to_string();
        tracing::info!(
            file = file!(),
            line = line!(),
            function = std::any::type_name::<fn()>(),
            method = %method,
            path = %path,
            query_string = %query_string,
            "Received request"
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
