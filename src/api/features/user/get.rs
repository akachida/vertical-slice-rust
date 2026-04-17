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
use crate::{
    crate::api::features::user::get::GetUserQuery,
    infrastructure::{
        application_error_response::ApplicationErrorResponse,
        rest::request_validation::RequestValidation,
    },
};

impl RequestValidation for GetUserQuery {
    fn validate(&self) -> Result<(), ApplicationErrorResponse> {
        if self.id.is_nil() {
            return Err(ApplicationErrorResponse {
                message: "Error while validating GetUser request".to_string(),
                error_code: 400,
                details: vec!["ID should not be empty".to_string()],
                inner: None,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::{
        crate::api::features::user::get::GetUserQuery,
        infrastructure::rest::request_validation::RequestValidation,
    };

    #[test]
    pub fn validation_error_if_id_is_nil() {
        // arrange
        let query = GetUserQuery { id: Uuid::nil() };

        // act
        let sut = query.validate().unwrap_err();

        // assert
        assert_eq!(
            sut.message,
            "Error while validating GetUser request".to_string()
        );
        assert_eq!(sut.error_code, 400);
        assert_eq!(sut.details, vec!["ID should not be empty".to_string()]);
        assert!(sut.inner.is_none())
    }
}
use actix_web::{get, web, HttpResponse, Result};
use uuid::Uuid;

use crate::{
    crate::api::features::user::get::{GetUserQuery, GetUserQueryHandler},
    infrastructure::application_error_response::ApplicationErrorResponseTrait,
};

#[get("{id}")]
pub async fn execute(path: web::Path<(Uuid,)>) -> Result<HttpResponse> {
    let handler = GetUserQueryHandler::new().await;

    if handler.is_err() {
        return handler.unwrap_err().into_http_response();
    }

    let query = GetUserQuery {
        id: path.into_inner().0,
    };
    let response = handler.unwrap().handle(&query).await;

    if response.is_err() {
        return response.unwrap_err().into_http_response();
    }

    Ok(HttpResponse::Ok().body(serde_json::to_string(&response.unwrap())?))
}
