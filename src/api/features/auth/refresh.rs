use sea_orm::{DatabaseConnection, DbBackend, EntityTrait, Statement};
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

use entity::user::Entity as User;

#[derive(Debug, Deserialize)]
pub struct RefreshTokenQuery {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub auth_token: String,
}

#[derive(Debug)]
pub struct RefreshTokenQueryHandler {
    db_context: DatabaseConnection,
}

impl RefreshTokenQueryHandler {
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

        Ok(RefreshTokenQueryHandler {
            db_context: db_context.conn.unwrap(),
        })
    }

    pub async fn handle(
        &self,
        query: &RefreshTokenQuery,
    ) -> Result<RefreshTokenResponse, ApplicationErrorResponse> {
        if let Err(error) = query.validate() {
            return Err(error);
        }

        let decode = JwtHelper::decode_refresh_token(query.refresh_token.to_owned());

        if decode.is_err() {
            let error_msg = decode.unwrap_err().to_string();

            log::info!("Refresh token is not valid: {}", error_msg);

            return Err(ApplicationErrorResponse {
                message: format!("Refresh token is not valid: {}", error_msg).to_string(),
                error_code: 400,
                details: vec![],
                inner: None,
            });
        }

        let user_uuid = decode.unwrap().sub;
        let find_user = User::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"SELECT * FROM "user" WHERE "id" = $1::uuid"#,
                vec![user_uuid.into()],
            ))
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

        let auth_token = JwtHelper::generate(&user.unwrap().into_domain());

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

        Ok(RefreshTokenResponse {
            auth_token: auth_token.unwrap(),
        })
    }
}
use crate::{
    crate::api::features::auth::refresh::RefreshTokenQuery,
    infrastructure::application_error_response::ApplicationErrorResponse,
    infrastructure::rest::request_validation::RequestValidation,
};

impl RequestValidation for RefreshTokenQuery {
    fn validate(&self) -> Result<(), ApplicationErrorResponse> {
        if self.refresh_token.is_empty() {
            return Err(ApplicationErrorResponse {
                message: "Refresh token is empty".to_string(),
                error_code: 400,
                details: vec![],
                inner: None,
            });
        }

        Ok(())
    }
}
use actix_web::{post, HttpRequest, HttpResponse, Result};

use crate::{
    crate::api::features::auth::refresh::{RefreshTokenQuery, RefreshTokenQueryHandler},
    infrastructure::application_error_response::{
        ApplicationErrorResponse, ApplicationErrorResponseTrait,
    },
};

#[post("refresh")]
pub async fn execute(request: HttpRequest) -> Result<HttpResponse> {
    let refresh_cookie = request.cookie("refresh");

    if refresh_cookie.is_none() {
        return ApplicationErrorResponse {
            message: "Refresh token cookie not found".to_string(),
            error_code: 400,
            details: vec![],
            inner: None,
        }
        .into_http_response();
    }

    let handler = RefreshTokenQueryHandler::new().await;

    if handler.is_err() {
        return handler.unwrap_err().into_http_response();
    }

    let query = RefreshTokenQuery {
        refresh_token: refresh_cookie.unwrap().value().to_string(),
    };

    match handler.unwrap().handle(&query).await {
        Err(error) => error.into_http_response(),
        Ok(result) => Ok(HttpResponse::Ok().body(serde_json::to_string(&result.auth_token)?)),
    }
}
