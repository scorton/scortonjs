use actix_web::{HttpRequest, HttpResponse, Error, dev::ServiceRequest, dev::ServiceResponse};
use actix_web::middleware::ServiceRequestExt;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use actix_web::dev::{forward_ready, Service, Transform};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct AuthMiddleware {
    jwt_secret: String,
}

impl AuthMiddleware {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip auth for health check
        if req.path() == "/api/health" {
            return Box::pin(self.service.call(req));
        }

        let auth_header = req.headers().get("Authorization");
        let jwt_secret = self.jwt_secret.clone();

        match auth_header {
            Some(header) => {
                if let Ok(header_str) = header.to_str() {
                    if header_str.starts_with("Bearer ") {
                        let token = &header_str[7..];
                        let validation = Validation::new(Algorithm::HS256);
                        
                        match decode::<Claims>(token, &DecodingKey::from_secret(jwt_secret.as_ref()), &validation) {
                            Ok(_claims) => {
                                // Token is valid, continue with the request
                                Box::pin(self.service.call(req))
                            }
                            Err(_) => {
                                // Token is invalid
                                Box::pin(async move {
                                    Ok(req.into_response(
                                        HttpResponse::Unauthorized().json(serde_json::json!({
                                            "success": false,
                                            "error": "Invalid token",
                                            "timestamp": chrono::Utc::now()
                                        }))
                                    ))
                                })
                            }
                        }
                    } else {
                        Box::pin(async move {
                            Ok(req.into_response(
                                HttpResponse::Unauthorized().json(serde_json::json!({
                                    "success": false,
                                    "error": "Invalid authorization header format",
                                    "timestamp": chrono::Utc::now()
                                }))
                            ))
                        })
                    }
                } else {
                    Box::pin(async move {
                        Ok(req.into_response(
                            HttpResponse::Unauthorized().json(serde_json::json!({
                                "success": false,
                                "error": "Invalid authorization header",
                                "timestamp": chrono::Utc::now()
                            }))
                        ))
                    })
                }
            }
            None => {
                Box::pin(async move {
                    Ok(req.into_response(
                        HttpResponse::Unauthorized().json(serde_json::json!({
                            "success": false,
                            "error": "Missing authorization header",
                            "timestamp": chrono::Utc::now()
                        }))
                    ))
                })
            }
        }
    }
}
