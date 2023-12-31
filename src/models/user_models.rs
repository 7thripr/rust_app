use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserModel {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserModel {
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUsername {
    pub username: String
}