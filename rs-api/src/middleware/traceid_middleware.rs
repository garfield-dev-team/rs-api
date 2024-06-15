use actix_service::{Service, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures_util::future::{ok, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing_actix_web::RequestId;
// use std::future::{ready, Ready as StdReady};

pub struct AddTraceIdToResponse;

impl<S, B> Transform<S, ServiceRequest> for AddTraceIdToResponse
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AddTraceIdToResponseMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AddTraceIdToResponseMiddleware { service })
    }
}

pub struct AddTraceIdToResponseMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AddTraceIdToResponseMiddleware<S>
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
        // 获取当前 span 的 trace-id
        let span = tracing::Span::current().id();

        // 提取 request_id，注意需要传入 `RequestId` 类型进行提取
        let request_id = req.extensions().get::<RequestId>().cloned();

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            // 获取当前 span 的 trace-id
            if let Some(span) = span {
                let trace_id = span.into_u64().to_string();
                res.headers_mut().insert(
                    HeaderName::from_static("x-trace-id"),
                    HeaderValue::from_str(&trace_id).unwrap(),
                );
            }

            // 将提取的 request_id 添加到响应头
            if let Some(request_id) = request_id {
                res.headers_mut().insert(
                    HeaderName::from_static("x-request-id"),
                    HeaderValue::from_str(&request_id.to_string()).unwrap(),
                );
            }

            Ok(res)
        })
    }
}
