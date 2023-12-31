use axum::{Json, http::StatusCode, extract::Path, Extension};
use entity::user::{ActiveModel, Entity, Column};
use sea_orm::{DatabaseConnection, Set, Condition, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};

use crate::{models::user_models::{UpdateUsername, GetUser}, utils::api_error::APIError};

pub async fn update_username(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<String>,
    Json(user_data): Json<UpdateUsername>,
) -> Result<(), APIError> {

    let mut user: ActiveModel = Entity::find()
    .filter(
        Condition::all() 
        .add(Column::Uuid.eq(uuid))
    ).one(&db)
    .await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
    .ok_or(APIError { message: "User not found".to_string(), status_code:StatusCode::NOT_FOUND, error_code: Some(41)})? // Convert Option<ActiveModel> to Result<ActiveModel, APIError>
    .into(); // Convert Entity to ActiveModel


    user.name = Set(user_data.username.to_owned());
    user.update(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    Ok(())
}

pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<String>,) -> Result<(), APIError> {
    
    let user = Entity::find()
    .filter(Column::Uuid.eq(uuid)).one(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
    .ok_or(APIError { message: "User not found".to_string(), status_code:StatusCode::NOT_FOUND, error_code: Some(41)})?; // Convert Option<ActiveModel> to Result<ActiveModel, APIError>

    Entity::delete_by_id(user.id).exec(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    Ok(())
}

pub async fn all_users(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<GetUser>>, APIError> {
    
    let users: Vec<GetUser> = Entity::find().all(&db).await.unwrap().into_iter().map(|item|GetUser{
            username: item.name,
            email: item.email,
        }
    ).collect();
    
    Ok(Json(users))
}