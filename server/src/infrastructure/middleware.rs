use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};

use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use crate::infrastructure::jwt::JwtService;
use uuid::Uuid;

pub struct AuthMiddleware {
    jwt_service: Rc<JwtService>,
}

impl AuthMiddleware {
    pub fn new(jwt_service: JwtService) -> Self {
        Self {
            jwt_service: Rc::new(jwt_service),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareInner {
            service,
            jwt_service: self.jwt_service.clone(),
        }))
    }
}

pub struct AuthMiddlewareInner<S> {
    service: S,
    jwt_service: Rc<JwtService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_service = self.jwt_service.clone();
        
        // Извлекаем токен из заголовка Authorization
        let auth_header = req.headers().get("Authorization");
        
        if let Some(auth_value) = auth_header {
            if let Ok(auth_str) = auth_value.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    
                    match jwt_service.verify_token(token) {
                        Ok(token_data) => {
                            // Добавляем user_id в расширения запроса
                            if let Ok(user_id) = Uuid::parse_str(&token_data.claims.sub) {
                                req.extensions_mut().insert(user_id);
                            }
                        }
                        Err(_) => {
                            // Недействительный токен
                            let (req, _pl) = req.into_parts();
                            let response = HttpResponse::Unauthorized()
                                .json(serde_json::json!({"error": "Недействительный токен"}));
                            return Box::pin(async { 
                                Ok(ServiceResponse::new(req, response)) 
                            });
                        }
                    }
                } else {
                    // Неверный формат заголовка
                    let (req, _pl) = req.into_parts();
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({"error": "Неверный формат токена"}));
                    return Box::pin(async { 
                        Ok(ServiceResponse::new(req, response)) 
                    });
                }
            }
        } else {
            // Отсутствует заголовок авторизации
            let (req, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({"error": "Требуется авторизация"}));
            return Box::pin(async { 
                Ok(ServiceResponse::new(req, response)) 
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

// Функция-помощник для извлечения user_id из запроса
pub fn extract_user_id(req: &ServiceRequest) -> Option<Uuid> {
    req.extensions().get::<Uuid>().copied()
}
