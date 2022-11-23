use crate::{
    infrastructure::security::jwt::jwt_helper::JwtHelper,
    presentation::rest::default_response::DefaultResponse,
};
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::Method,
    Error, HttpMessage, HttpResponse,
};
use futures::future::LocalBoxFuture;
use log::info;
use std::future::{ready, Ready};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let public_routes = vec![
            "/",
            "/api",
            "/api/auth",
            "/api/auth/refresh",
            "/api/auth/validate",
        ];
        let req_path = &request.path();

        if public_routes.contains(req_path) || *request.method() == Method::OPTIONS {
            let response = self.service.call(request);
            return Box::pin(
                async move { response.await.map(ServiceResponse::map_into_left_body) },
            );
        }

        let authorization = request.headers().get("Authorization");

        if authorization.is_none() {
            info!("Unauthorized Access: No authorization header");
            return Box::pin(async move { Ok(unauthorized_access(request).map_into_right_body()) });
        }

        let auth_header_value = authorization
            .unwrap()
            .to_str()
            .unwrap_or("")
            .trim()
            .to_string();

        if auth_header_value.is_empty() {
            info!("Unauthorized Access: Authorization header is empty");
            return Box::pin(async move { Ok(unauthorized_access(request).map_into_right_body()) });
        }

        if !auth_header_value.starts_with("Bearer") && !auth_header_value.starts_with("bearer") {
            info!("Unauthorized Access: Invalid value for Authorization header");
            return Box::pin(async move { Ok(unauthorized_access(request).map_into_right_body()) });
        }

        let token_string = auth_header_value[6..].trim();

        match JwtHelper::decode_token(token_string.to_string()) {
            Err(jwt_error) => {
                info!("Unauthorized Access: {}", jwt_error);
                Box::pin(async move { Ok(unauthorized_access(request).map_into_right_body()) })
            }
            Ok(user_claims) => {
                // Insert the UserClaims into the request
                request.extensions_mut().insert(user_claims);
                let response = self.service.call(request);
                Box::pin(async move { response.await.map(ServiceResponse::map_into_left_body) })
            }
        }
    }
}

pub fn unauthorized_access(request: ServiceRequest) -> ServiceResponse {
    let (request, _pl) = request.into_parts();

    let response =
        HttpResponse::Unauthorized().json(DefaultResponse::new("Unauthorized Access", ""));

    ServiceResponse::new(request, response)
}
