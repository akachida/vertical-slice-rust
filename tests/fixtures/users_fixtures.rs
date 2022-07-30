use chrono::Utc;
use entity::user;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, Set};

pub async fn users_fixtures(context: &DatabaseConnection) {
    user::ActiveModel {
        id: Set(Uuid::new_v4()),
        first_name: Set("Admin".to_string()),
        last_name: Set("Master".to_string()),
        email: Set("admin@master.com".to_string()),
        role_id: Set(1),
        hashed_password: Set("$argon2id$v=19$m=65536,t=2,p=1$67ead7409f9eacd9a38c02464c601cbce20ba9ce5a3460f1c7c6396039047ca1$xIPW7JUKHLks0xSwWKAeu6BV6qCDKwbOMwscm1+6MvI".to_string()),
        is_active: Set(Some(true)),
        is_admin: Set(Some(true)),
        updated_at: Set(None),
        created_at: Set(Some(Utc::now())),
        last_login_at: Set(None),
    }
    .insert(context)
    .await
    .unwrap();
}
