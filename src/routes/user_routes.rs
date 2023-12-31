use axum::{Router, http, routing::{get, put}};
use tower_http::cors::{CorsLayer, Any};
use http::Method;

use crate::handler::user_handlers::{update_username, delete_user, all_users};

pub fn user_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::PUT, Method::DELETE, Method::POST])
        .allow_origin(Any);
    
    let router = Router::new()
    .route("/api/user/:uuid/update", put(update_username),)
    .route("/api/user/:uuid/delete", get(delete_user),)
    .route("/api/users", get(all_users),)
    .layer(cors);
    router
}