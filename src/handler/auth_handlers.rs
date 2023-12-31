use axum::{Json, response::IntoResponse, http::StatusCode, Extension};
use entity::user::{ActiveModel, Entity};
use sea_orm::{DatabaseConnection, Set, Condition, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};
use uuid::Uuid;

use crate::models::user_models::{CreateUserModel, LoginUserModel, User};

pub async fn register(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>,
) -> impl IntoResponse {

    let user_model: ActiveModel = ActiveModel {
        name: Set(user_data.username.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4().to_string()),
        ..Default::default()
    };
    user_model.insert(&db).await.unwrap();

    (StatusCode::ACCEPTED, "Data Inserted")
}

pub async fn login_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserModel>,
) -> impl IntoResponse {
    
    let user = Entity::find()
    .filter(
        Condition::all() 
        .add(entity::user::Column::Email.eq(user_data.email))
        .add(entity::user::Column::Password.eq(user_data.password))
    ).one(&db)
    .await.unwrap().unwrap();

    let data = User{
        username: user.name,
        email: user.email,
        uuid: user.uuid,
        password: user.password,
    };

    (StatusCode::ACCEPTED, Json(data))
    
}