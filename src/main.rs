#![allow(non_snake_case, unused)]
mod models;
mod routes;
mod handler;
mod utils;

use axum::{
    routing::get,
    http::{StatusCode, Request}, Router,
    response::{IntoResponse, Response, Html},
    middleware::{self, Next}, 
    body::Body, Extension,
};
use routes::{auth_routes::auth_routes, user_routes::user_routes};
use sea_orm::Database;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::consts::DATABASE_URL;

use models::webpage_models::FormTemplate;

use serde_json::json;
use askama::Template;

use crate::models::user_models::UpdateUsername;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server() {

    let db_url = DATABASE_URL.clone();

    let db = Database::connect(db_url).await
    .expect("Failed to connect to database");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(index),)
        .route("/api/status", get(status),)
        .merge(auth_routes())
        .merge(user_routes())
        .layer(Extension(db))
        .layer(middleware::from_fn(logging_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app)
    .await
    .map_err(|error| eprintln!("Server error: {}", error));

}

async fn logging_middleware(req: Request<Body>, next: Next) -> impl IntoResponse {
    println!("Received a request to {}", req.uri());
    next.run(req).await
}

fn render_template(template: impl Template) -> Response {
    match template.render() {
        Ok(rendered) => Html(rendered).into_response(),
        Err(e) => {
            eprintln!("Failed to render template: {e:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn status() -> impl IntoResponse {
    let status = "online";
    (StatusCode::ACCEPTED, status)
}

async fn index() -> Response {
    // Default with empty strings.
    let template = FormTemplate::default();
    render_template(template)
}