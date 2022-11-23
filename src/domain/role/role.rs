use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Role {
    id: i16,
    title: String,
    description: Option<String>,
    is_active: bool,
    updated_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl Role {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: 0,
            title,
            description,
            is_active: true,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn id(&self) -> &i16 {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn title_mut(&mut self, title: String) {
        self.title = title;
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn description_mut(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn is_active(&self) -> &bool {
        &self.is_active
    }

    pub fn is_active_mut(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &Option<DateTime<Utc>> {
        &self.updated_at
    }

    pub fn updated_at_mut(&mut self, updated_at: DateTime<Utc>) {
        self.updated_at = Some(updated_at);
    }
}

pub trait RoleTrait {
    fn into_domain(self) -> Role;
}

impl RoleTrait for entity::role::Model {
    fn into_domain(self) -> Role {
        Role {
            id: self.id,
            title: self.title,
            description: self.description,
            is_active: self.is_active,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}
