use axum::{Json, http::StatusCode, Extension};
use entity::user::{ActiveModel, Entity};
use sea_orm::{DatabaseConnection, Set, Condition, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait};
use uuid::Uuid;

use crate::{models::user_models::{CreateUserModel, LoginUserModel, User}, utils::api_error::APIError};

pub async fn register(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>,
) -> Result<(), APIError> {
    // check if the user exist
    let user = entity::user::Entity::find()
    .filter(entity::user::Column::Email.eq(user_data.email.clone()))
    .one(&db).await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

    if user != None {
        return  Err(APIError { message: "User exists".to_owned(), status_code:StatusCode::CONFLICT, error_code: Some(40) });
    }

    let user_model: ActiveModel = ActiveModel {
        name: Set(user_data.username.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4().to_string()),
        ..Default::default()
    };
    user_model.insert(&db).await.unwrap();

    Ok(())
}

pub async fn login_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserModel>,
) -> Result<Json<User>, APIError> {
    
    let user = Entity::find()
    .filter(
        Condition::all() 
        .add(entity::user::Column::Email.eq(user_data.email))
        .add(entity::user::Column::Password.eq(user_data.password))
    ).one(&db)
    .await
    .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
    .ok_or(APIError { message: "User not found".to_owned(), status_code:StatusCode::NOT_FOUND, error_code: Some(40) })?;

    let data = User{
        username: user.name,
        email: user.email,
        uuid: user.uuid,
        password: user.password,
    };
    Ok(Json(data))
}