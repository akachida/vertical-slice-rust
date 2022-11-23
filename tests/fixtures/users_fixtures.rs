use backend_api::domain::value_objects::hashed_password::HashedPassword;
use chrono::Utc;
use entity::user;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, Set};

pub async fn users_fixtures(context: &DatabaseConnection) {
    user::ActiveModel {
        id: Set(Uuid::parse_str("425123f7-49c6-44f4-b39e-31477092a8c3").unwrap()),
        first_name: Set("Admin".to_string()),
        last_name: Set("Master".to_string()),
        email: Set("admin@master.com".to_string()),
        role_id: Set(1),
        hashed_password: Set(HashedPassword::new("1aBcD!fg2@").unwrap().to_string()),
        is_active: Set(true),
        is_admin: Set(true),
        updated_at: Set(None),
        created_at: Set(Utc::now()),
        last_login_at: Set(None),
    }
    .insert(context)
    .await
    .unwrap();
}
