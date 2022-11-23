use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::{
        user::user::User,
        value_objects::{email::Email, hashed_password::HashedPassword},
    },
    infrastructure::{
        application_error_response::{
            ApplicationErrorResponse, DefaultApplicationErrorResponseTrait,
        },
        persistence::{
            database_context::{ReadDbContext, WriteDbContext},
            database_manager::DatabaseManager,
        },
        rest::request_validation::RequestValidation,
    },
};

use entity::user;
use entity::user::Entity as UserEntity;

#[derive(Debug, Deserialize)]
pub struct CreateUserCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role_id: i16,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateUserCommandResponse {
    pub id: Uuid,
}

#[derive(Debug)]
pub struct CreateUserCommandHandler {
    read_context: DatabaseConnection,
    write_context: DatabaseConnection,
}

impl CreateUserCommandHandler {
    pub async fn new() -> Result<Self, ApplicationErrorResponse> {
        let read_context = ReadDbContext::new().await;
        let write_context = WriteDbContext::new().await;

        if read_context.conn.is_none() {
            return Err(ApplicationErrorResponse {
                message: "Error while trying to connect to database".to_string(),
                error_code: 500,
                details: vec!["Failed to connect to Read context".to_string()],
                inner: None,
            });
        }

        if write_context.conn.is_none() {
            return Err(ApplicationErrorResponse {
                message: "Error while trying to connect to database".to_string(),
                error_code: 500,
                details: vec!["Failed to connect to Write context".to_string()],
                inner: None,
            });
        }

        Ok(Self {
            read_context: read_context.conn.unwrap(),
            write_context: write_context.conn.unwrap(),
        })
    }

    pub async fn handle(
        self,
        command: &CreateUserCommand,
    ) -> Result<CreateUserCommandResponse, ApplicationErrorResponse> {
        if let Err(error) = command.validate() {
            return Err(error);
        }

        let existing_user = UserEntity::find()
            .filter(user::Column::Email.contains(command.email.as_str()))
            .one(&self.read_context)
            .await;

        if existing_user.is_err() {
            return Err(existing_user.unwrap_err().into_application_error_response());
        }

        if existing_user.unwrap().is_some() {
            return Err(ApplicationErrorResponse {
                message: "User already exists".to_string(),
                error_code: 400,
                details: vec![],
                inner: None,
            });
        }

        let new_user = User::new(
            &command.first_name,
            &command.last_name,
            Email::new(&command.email).unwrap(),
            command.role_id,
            HashedPassword::new(&command.password).unwrap(),
            command.is_admin,
        );

        let insert_user = new_user
            .into_active_model()
            .insert(&self.write_context)
            .await;

        if insert_user.is_err() {
            return Err(insert_user.unwrap_err().into_application_error_response());
        }

        Ok(CreateUserCommandResponse {
            id: new_user.id().to_owned(),
        })
    }
}
