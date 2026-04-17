use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    domain::user::user::UserTrait,
    infrastructure::{
        application_error_response::{
            ApplicationErrorResponse, DefaultApplicationErrorResponseTrait,
        },
        persistence::{database_context::ReadDbContext, database_manager::DatabaseManager},
        rest::request_validation::RequestValidation,
        security::jwt::jwt_helper::JwtHelper,
    },
};

use entity::user;
use entity::user::Entity as User;

#[derive(Debug, Deserialize)]
pub struct AuthLoginQuery {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthLoginQueryResponse {
    pub auth_token: String,
    pub refresh_token: String,
}

#[derive(Debug)]
pub struct AuthLoginQueryHandler {
    db_context: DatabaseConnection,
}

impl AuthLoginQueryHandler {
    pub async fn new() -> Result<Self, ApplicationErrorResponse> {
        let db_context = ReadDbContext::new().await;

        if db_context.conn.is_none() {
            return Err(ApplicationErrorResponse {
                message: "Error while trying to connect to database".to_string(),
                error_code: 500,
                details: vec![],
                inner: None,
            });
        }

        Ok(AuthLoginQueryHandler {
            db_context: db_context.conn.unwrap(),
        })
    }

    pub async fn handle(
        &self,
        query: &AuthLoginQuery,
    ) -> Result<AuthLoginQueryResponse, ApplicationErrorResponse> {
        if let Err(error) = query.validate() {
            return Err(error);
        }

        let find_user = User::find()
            .filter(user::Column::Email.contains(query.username.as_str()))
            .one(&self.db_context)
            .await;

        if find_user.is_err() {
            return Err(find_user.unwrap_err().into_application_error_response());
        }

        let user = find_user.unwrap();

        if user.is_none() {
            return Err(ApplicationErrorResponse {
                message: "Credentials were invalid".to_string(),
                error_code: 401,
                details: vec!["Account not found".to_string()],
                inner: None,
            });
        }

        let user_model = user.unwrap().into_domain();

        if !user_model.hashed_password().verify(query.password.as_str()) {
            return Err(ApplicationErrorResponse {
                message: "Credentials were invalid".to_string(),
                error_code: 401,
                details: vec!["Username or password were invalid".to_string()],
                inner: None,
            });
        }

        let auth_token = JwtHelper::generate(&user_model);
        let refresh_token = JwtHelper::generate_refresh_token(&user_model);

        if auth_token.is_err() {
            let jwt_error = auth_token.unwrap_err();

            log::error!(
                "Error while generating authentication token: {}",
                jwt_error.to_string()
            );

            return Err(ApplicationErrorResponse {
                message: jwt_error.to_string(),
                error_code: 500,
                details: vec![],
                inner: None,
            });
        }

        if refresh_token.is_err() {
            let jwt_error = refresh_token.unwrap_err();

            log::error!(
                "Error while generating a refresh token: {}",
                jwt_error.to_string()
            );

            return Err(ApplicationErrorResponse {
                message: jwt_error.to_string(),
                error_code: 500,
                details: vec![],
                inner: None,
            });
        }

        Ok(AuthLoginQueryResponse {
            auth_token: auth_token.unwrap(),
            refresh_token: refresh_token.unwrap(),
        })
    }
}
use crate::{
    crate::api::features::auth::login::AuthLoginQuery,
    infrastructure::application_error_response::ApplicationErrorResponse,
    infrastructure::rest::request_validation::RequestValidation,
};

impl RequestValidation for AuthLoginQuery {
    fn validate(&self) -> Result<(), ApplicationErrorResponse> {
        let mut error_messages: Vec<String> = Vec::new();

        if self.username.is_empty() {
            error_messages.append(&mut vec!["Username is required".to_string()]);
        }

        if self.password.is_empty() {
            error_messages.append(&mut vec!["Password is required".to_string()]);
        }

        if !error_messages.is_empty() {
            return Err(ApplicationErrorResponse {
                message: "Username or password was not valid".to_string(),
                error_code: 400,
                details: error_messages,
                inner: None,
            });
        }

        Ok(())
    }
}
use actix_web::{cookie::Cookie, post, web, HttpResponse, Result};
use std::ops::Deref;

use crate::{
    crate::api::features::auth::login::{AuthLoginQuery, AuthLoginQueryHandler},
    infrastructure::application_error_response::*,
};

#[post("")]
pub async fn execute(query: web::Json<AuthLoginQuery>) -> Result<HttpResponse> {
    let handler = AuthLoginQueryHandler::new().await;

    if handler.is_err() {
        return handler.unwrap_err().into_http_response();
    }

    match handler.unwrap().handle(query.deref()).await {
        Err(error) => error.into_http_response(),
        Ok(result) => Ok(HttpResponse::Ok()
            .cookie(
                Cookie::build("refresh", &result.refresh_token)
                    .secure(true)
                    .http_only(true)
                    .finish(),
            )
            .body(serde_json::to_string(&result.auth_token)?)),
    }
}
