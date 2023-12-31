#![allow(unused_imports)]

use axum::{
    routing::{get, post},
    http::{StatusCode, Request},
    Json, Router,
    response::IntoResponse,
    middleware::{self, Next}, 
    body::Body,
};

use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::{SubscriberExt, self}, util::SubscriberInitExt, fmt::layer};
use sea_orm::{
    Database,
    DatabaseConnection,
    Set, ActiveModelTrait,
};
use entity::user::{Entity, ActiveModel, self};
use uuid::Uuid;

mod models;
use models::user_models::{CreateUserModel, User};

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
    .route("/api/create_user", post(create_user_post),)
    .layer(middleware::from_fn(logging_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app)
    .await.unwrap();

}

async fn logging_middleware(req: Request<Body>, next: Next) -> impl IntoResponse {
    println!("Received a request to {}", req.uri());
    next.run(req).await
}

async fn test() -> impl IntoResponse {
    println!("First Api");

    (StatusCode::ACCEPTED, "Api Working")
}

async fn create_user_post(
    Json(user_data): Json<CreateUserModel>,
) -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("sqlite:///Users/rishabhprakash/Rust/axum_learn/proejctDB.db")
    .await.unwrap();
    
    let user_model: ActiveModel = ActiveModel {
        name: Set(user_data.username.to_owned()),
        email: Set(user_data.email.to_owned()),
        password: Set(user_data.password.to_owned()),
        uuid: Set(Uuid::new_v4().to_string()),
        ..Default::default()
    };
    println!("User Data: {:?}", user_model);
    user_model.insert(&db).await.unwrap();
    db.close().await;
    (StatusCode::ACCEPTED, "Data Inserted")
}