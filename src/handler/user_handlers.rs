use axum::{Json, response::IntoResponse, http::StatusCode, extract::Path, Extension};
use entity::user::{ActiveModel, Entity, Column};
use sea_orm::{DatabaseConnection, Set, Condition, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};

use crate::models::user_models::{UpdateUsername, GetUser};

pub async fn update_username(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<String>,
    Json(user_data): Json<UpdateUsername>,
) -> impl IntoResponse {

    let mut user: ActiveModel = Entity::find()
    .filter(
        Condition::all() 
        .add(Column::Uuid.eq(uuid))
    ).one(&db)
    .await.unwrap().unwrap().into(); // Convert Entity to ActiveModel
    user.name = Set(user_data.username.to_owned());
    user.update(&db).await.unwrap();

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, "Data Updated")
}

pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<String>,) -> impl IntoResponse {
    
    let user = Entity::find()
    .filter(Column::Uuid.eq(uuid)).one(&db).await.unwrap().unwrap();

    Entity::delete_by_id(user.id).exec(&db).await.unwrap();

    db.close().await.unwrap();

    (StatusCode::ACCEPTED, "Deleted User")
}

pub async fn all_users(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    
    let users: Vec<GetUser> = Entity::find().all(&db).await.unwrap().into_iter().map(|item|GetUser{
            username: item.name,
            email: item.email,
        }
    ).collect();

    db.close().await.unwrap();

    (StatusCode::ACCEPTED, Json(users))
}