#![allow(unused_imports)]

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sea_orm::{
    Database,
    DatabaseConnection,
    Set, ActiveModelTrait,
};
use entity::user::{Entity, ActiveModel};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {

    tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app = Router::new()
    .route("/api/test", get(test),)
    .route("/api/create_user", post(create_user),);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app)
    .await.unwrap();

}

async fn test() -> impl IntoResponse {
    println!("First Api");

    (StatusCode::ACCEPTED, "Api Working")
}

async fn create_user() -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("sqlite:///Users/rishabhprakash/Rust/axum_learn/proejctDB.db")
    .await.unwrap();

    let user_model: ActiveModel = ActiveModel {
        name: Set("test".to_owned()),
        email: Set("test@os.com".to_owned()),
        password: Set("test".to_owned()),
        uuid: Set(Uuid::new_v4().to_string()),
        ..Default::default()
    };
    let usr = user_model.insert(&db).await.unwrap();

    (StatusCode::ACCEPTED, "Data Inserted")
}