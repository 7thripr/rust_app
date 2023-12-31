use axum::{
    routing::{get, post, put, delete},
    http::{StatusCode, Request},
    Json, Router,
    response::IntoResponse,
    middleware::{self, Next}, 
    body::Body, extract::Path,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sea_orm::{
    Database,
    DatabaseConnection,
    Set, ActiveModelTrait, Condition, ColumnTrait, EntityTrait, QueryFilter,
};
use entity::user::{ActiveModel,
    Entity, Column};
use uuid::Uuid;

mod models;
use models::user_models::{CreateUserModel, User, LoginUserModel, UpdateUsername};

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {

    tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .init();

    let app = Router::new()
    .route("/api/status", get(status),)
    .route("/api/create_user", post(create_user_post),)
    .route("/api/login", get(login_user),)
    .route("/api/user/:uuid/update", put(update_username),)
    .route("/api/user/:uuid/delete", delete(delete_user),)
    .layer(middleware::from_fn(logging_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app)
    .await.unwrap();

}

async fn logging_middleware(req: Request<Body>, next: Next) -> impl IntoResponse {
    println!("Received a request to {}", req.uri());
    next.run(req).await
}

async fn status() -> impl IntoResponse {
    let status = "online";
    (StatusCode::ACCEPTED, status)
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
    user_model.insert(&db).await.unwrap();
    db.close().await.unwrap();
    (StatusCode::ACCEPTED, "Data Inserted")
}

async fn login_user(
    Json(user_data): Json<LoginUserModel>,
) -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("sqlite:///Users/rishabhprakash/Rust/axum_learn/proejctDB.db")
    .await.unwrap();
    
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

    db.close().await.unwrap();
    (StatusCode::ACCEPTED, Json(data))
    
}

async fn update_username(
    Path(uuid): Path<String>,
    Json(user_data): Json<UpdateUsername>,
) -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("sqlite:///Users/rishabhprakash/Rust/axum_learn/proejctDB.db")
    .await.unwrap();
    
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

async fn delete_user(
    Path(uuid): Path<String>,) -> impl IntoResponse {
    let db: DatabaseConnection = Database::connect("sqlite:///Users/rishabhprakash/Rust/axum_learn/proejctDB.db").await.unwrap();
    
    let user = Entity::find()
    .filter(Column::Uuid.eq(uuid)).one(&db).await.unwrap().unwrap();

    Entity::delete_by_id(user.id).exec(&db).await.unwrap();

    db.close().await.unwrap();

    (StatusCode::ACCEPTED, "Deleted User")
}