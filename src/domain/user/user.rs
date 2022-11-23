use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::{prelude::Uuid as SeaUuid, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::{email::Email, hashed_password::HashedPassword};

use entity::user::ActiveModel as UserActiveModel;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: Email,
    role_id: i16,
    hashed_password: HashedPassword,
    is_active: bool,
    is_admin: bool,
    updated_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    last_login_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        first_name: &str,
        last_name: &str,
        email: Email,
        role_id: i16,
        hashed_password: HashedPassword,
        is_admin: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email,
            role_id,
            hashed_password,
            is_active: true,
            is_admin,
            updated_at: None,
            created_at: Utc::now(),
            last_login_at: None,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn first_name_mut(&mut self, value: String) {
        self.first_name = value;
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn last_name_mut(&mut self, value: String) {
        self.last_name = value;
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn email_mut(&mut self, value: Email) {
        self.email = value;
    }

    pub fn role_id(&self) -> &i16 {
        &self.role_id
    }

    pub fn role_id_mut(&mut self, value: i16) {
        self.role_id = value;
    }

    pub fn hashed_password(&self) -> &HashedPassword {
        &self.hashed_password
    }

    pub fn hashed_password_mut(&mut self, value: HashedPassword) {
        self.hashed_password = value;
    }

    pub fn is_active(&self) -> &bool {
        &self.is_active
    }

    pub fn toggle_is_active(&mut self, value: bool) {
        self.is_active = value;
    }

    pub fn is_admin(&self) -> &bool {
        &self.is_admin
    }

    pub fn toggle_is_admin(&mut self, value: bool) {
        self.is_admin = value;
    }

    pub fn updated_at(&self) -> &Option<DateTime<Utc>> {
        &self.updated_at
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn last_login_at(&self) -> &Option<DateTime<Utc>> {
        &self.last_login_at
    }

    pub fn update_last_login_at(&mut self) {
        self.last_login_at = Some(Utc::now());
    }

    pub fn into_active_model(&self) -> UserActiveModel {
        UserActiveModel {
            id: Set(SeaUuid::new_v4()),
            first_name: Set(self.first_name.to_owned()),
            last_name: Set(self.last_name.to_owned()),
            email: Set(self.email.to_string()),
            role_id: Set(self.role_id),
            hashed_password: Set(self.hashed_password.to_string()),
            is_active: Set(self.is_active),
            is_admin: Set(self.is_admin),
            updated_at: Set(None),
            created_at: Set(DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
                Utc,
            )),
            last_login_at: Set(None),
        }
    }
}

pub trait UserTrait {
    fn into_domain(self) -> User;
}

impl UserTrait for entity::user::Model {
    fn into_domain(self) -> User {
        User {
            id: Uuid::from_bytes(self.id.as_bytes().to_owned()),
            first_name: self.first_name,
            last_name: self.last_name,
            email: Email::new(self.email.as_str()).unwrap(),
            role_id: self.role_id,
            hashed_password: HashedPassword::new_from_hash(self.hashed_password.as_str()).unwrap(),
            is_active: self.is_active,
            is_admin: self.is_admin,
            updated_at: self.updated_at,
            created_at: self.created_at,
            last_login_at: self.last_login_at,
        }
    }
}
