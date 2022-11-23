use chrono::{DateTime, Utc};
use sea_orm::{prelude::Uuid as SeaOrmUuid, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::user::user::UserTrait,
    infrastructure::{
        application_error_response::{
            ApplicationErrorResponse, DefaultApplicationErrorResponseTrait,
        },
        persistence::{database_context::ReadDbContext, database_manager::DatabaseManager},
        rest::request_validation::RequestValidation,
    },
};

use entity::user::Entity as UserEntity;

#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct GetUserQueryResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role_id: i16,
    pub is_active: bool,
    pub is_admin: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct GetUserQueryHandler {
    context: DatabaseConnection,
}

impl GetUserQueryHandler {
    pub async fn new() -> Result<Self, ApplicationErrorResponse> {
        let read_context = ReadDbContext::new().await;

        if read_context.conn.is_none() {
            return Err(ApplicationErrorResponse {
                message: "Error while trying to connect to database".to_string(),
                error_code: 500,
                details: vec!["Failed to connect to Read context".to_string()],
                inner: None,
            });
        }

        Ok(Self {
            context: read_context.conn.unwrap(),
        })
    }

    pub async fn handle(
        self,
        query: &GetUserQuery,
    ) -> Result<GetUserQueryResponse, ApplicationErrorResponse> {
        if let Err(validation_error) = query.validate() {
            return Err(validation_error);
        }

        let db_uuid = SeaOrmUuid::from_bytes(query.id.as_bytes().to_owned());
        let existing_user_query = UserEntity::find_by_id(db_uuid).one(&self.context).await;

        if existing_user_query.is_err() {
            return Err(existing_user_query
                .unwrap_err()
                .into_application_error_response());
        }

        let user_result = existing_user_query.unwrap();

        if user_result.is_none() {
            return Err(ApplicationErrorResponse {
                message: "User not found".to_string(),
                error_code: 404,
                details: vec![],
                inner: None,
            });
        }

        let user = user_result.unwrap().into_domain();

        Ok(GetUserQueryResponse {
            id: user.id().to_owned(),
            first_name: user.first_name().to_owned(),
            last_name: user.last_name().to_owned(),
            email: user.email().to_string(),
            role_id: user.role_id().to_owned(),
            is_active: user.is_active().to_owned(),
            is_admin: user.is_admin().to_owned(),
            updated_at: user.updated_at().to_owned(),
            created_at: user.created_at().to_owned(),
            last_login_at: user.last_login_at().to_owned(),
        })
    }
}
