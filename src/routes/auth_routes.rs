use axum::{Router, http, routing::{post, get}};
use tower_http::cors::{CorsLayer, Any};
use http::Method;

use crate::handler::auth_handlers::{register, login_user};


pub fn auth_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);
    
    let router = Router::new()
    .route("/api/register", post(register),)
    .route("/api/login", get(login_user),)
    .layer(cors);
    router
}