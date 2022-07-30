use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    domain::user::user::UserTrait,
    infrastructure::{
        security::jwt::jwt_helper::JwtHelper,
        persistence::{database_context::ReadDbContext, database_manager::DatabaseManager},
        rest::{
            application_error_response::ApplicationErrorResponse,
            request_validation::RequestValidation,
        },
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
    pub token: String,
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
            let error = find_user.unwrap_err();
            log::error!("Error while fetching requested data: {}", error.to_string());

            return Err(ApplicationErrorResponse {
                message: error.to_string(),
                error_code: 500,
                details: vec![],
                inner: None,
            });
        }

        let user = find_user.unwrap();

        if user.is_none() {
            return Err(ApplicationErrorResponse {
                message: "Credentials were invalid".to_string(),
                error_code: 401,
                details: vec![
                    "Username or password were invalid".to_string(),
                    "Account not found".to_string(),
                ],
                inner: None,
            });
        }

        match JwtHelper::generate(&user.unwrap().into_domain()) {
            Err(jwt_error) => {
                log::error!(
                    "Error while converting User data model into Domain model: {}",
                    jwt_error.to_string()
                );

                Err(ApplicationErrorResponse {
                    message: jwt_error.to_string(),
                    error_code: 500,
                    details: vec![],
                    inner: None,
                })
            }
            Ok(token) => Ok(AuthLoginQueryResponse { token }),
        }
    }
}
