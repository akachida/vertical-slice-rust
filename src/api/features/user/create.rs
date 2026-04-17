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
use crate::{
    crate::api::features::user::create::CreateUserCommand,
    domain::value_objects::{email::Email, hashed_password::HashedPassword},
    infrastructure::application_error_response::ApplicationErrorResponse,
    infrastructure::rest::request_validation::RequestValidation,
};

impl RequestValidation for CreateUserCommand {
    fn validate(&self) -> Result<(), ApplicationErrorResponse> {
        let mut error_messages: Vec<String> = Vec::new();

        if self.first_name.is_empty() {
            error_messages.append(&mut vec!["First name is empty".to_string()]);
        }

        if self.last_name.is_empty() {
            error_messages.append(&mut vec!["Last name is empty".to_string()]);
        }

        if self.email.is_empty() {
            error_messages.append(&mut vec!["Email is empty".to_string()]);
        }

        if self.role_id <= 0 {
            error_messages.append(&mut vec!["Role ID should be greater than 0".to_string()]);
        }

        if self.first_name.len() < 3 || self.first_name.len() > 30 {
            error_messages.append(&mut vec![
                "First name should have between 3 and 30 characters".to_string(),
            ]);
        }

        if self.last_name.len() < 3 || self.last_name.len() > 30 {
            error_messages.append(&mut vec![
                "Last name should have between 3 and 30 characters".to_string(),
            ]);
        }

        let valid_password = HashedPassword::validate(&self.password);

        if valid_password.is_err() {
            error_messages.append(&mut vec![valid_password.unwrap_err().to_string()]);
        }

        if !Email::validate(&self.email) {
            error_messages.append(&mut vec!["Email it's not a valid email address".to_string()]);
        }

        if !error_messages.is_empty() {
            return Err(ApplicationErrorResponse {
                message: "Error while trying to create a new User".to_string(),
                error_code: 400,
                details: error_messages,
                inner: None,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        crate::api::features::user::create::CreateUserCommand,
        infrastructure::rest::request_validation::RequestValidation,
    };

    #[test]
    pub fn error_with_all_fields_invalid() {
        // arrange
        let command = CreateUserCommand {
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            role_id: 0,
            password: "".to_string(),
            is_admin: false,
        };

        // act
        let sut = command.validate().unwrap_err();

        // assert
        assert_eq!(
            sut.message,
            "Error while trying to create a new User".to_string()
        );
        assert_eq!(sut.error_code, 400);
        assert_eq!(
            sut.details,
            vec![
                "First name is empty".to_string(),
                "Last name is empty".to_string(),
                "Email is empty".to_string(),
                "Role ID should be greater than 0".to_string(),
                "First name should have between 3 and 30 characters".to_string(),
                "Last name should have between 3 and 30 characters".to_string(),
                "Empty password".to_string(),
                "Email it's not a valid email address".to_string()
            ]
        );
        assert!(sut.inner.is_none());
    }
}
use actix_web::{post, web, HttpResponse, Result};

use crate::{
    crate::api::features::user::create::{
        CreateUserCommand, CreateUserCommandHandler,
    },
    infrastructure::application_error_response::ApplicationErrorResponseTrait,
};

#[post("")]
pub async fn execute(command: web::Json<CreateUserCommand>) -> Result<HttpResponse> {
    let handler = CreateUserCommandHandler::new().await;

    if handler.is_err() {
        return handler.unwrap_err().into_http_response();
    }

    let handle = handler.unwrap().handle(&command).await;

    if handle.is_err() {
        return handle.unwrap_err().into_http_response();
    }

    let response = handle.unwrap();
    // todo: create a URL location for resource created
    // let url_for = HttpRequest::url_for();

    Ok(HttpResponse::Created()
        .append_header(("Location", format!("/api/user/{}", response.id.to_owned())))
        .body(response.id.to_string()))
}
