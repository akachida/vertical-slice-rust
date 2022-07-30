use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::{email::Email, hashed_password::HashedPassword, role::Role};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: Email,
    role: Role,
    hashed_password: HashedPassword,
    is_active: bool,
    is_admin: bool,
    updated_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    last_login_at: Option<DateTime<Utc>>,
}

impl User {
    /// Returns a default creation of User
    pub fn new(
        first_name: &str,
        last_name: &str,
        email: Email,
        role: Role,
        hashed_password: HashedPassword,
        is_admin: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            email,
            role,
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

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn role_mut(&mut self, value: Role) {
        self.role = value;
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
            role: Role::from_i16(self.role_id),
            hashed_password: HashedPassword::new_from_hash(self.hashed_password.as_str()).unwrap(),
            is_active: self.is_active.unwrap_or(false),
            is_admin: self.is_admin.unwrap_or(false),
            updated_at: self.updated_at,
            created_at: self.created_at.unwrap(),
            last_login_at: self.last_login_at,
        }
    }
}
