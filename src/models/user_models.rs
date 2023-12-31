use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserModel {
    pub username: String,
    pub password: String,
    pub email: String,
}