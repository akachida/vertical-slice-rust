use chrono::Utc;
use entity::role;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn roles_fixtures(context: &DatabaseConnection) {
    role::ActiveModel {
        id: Set(1),
        title: Set("Admin".to_string()),
        description: Set(None),
        is_active: Set(true),
        updated_at: Set(None),
        created_at: Set(Utc::now()),
    }
    .insert(context)
    .await
    .unwrap();
}
