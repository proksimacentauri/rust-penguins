use std::sync::Arc;

use axum::{
    routing::{get, post, put, delete},
    Router, http::Method,
};
use tower_http::cors::{CorsLayer, Any};

use crate::{
    handlers::penguins::{ handle_hello, handle_get_penguins, handle_get_penguin_by_id, handle_post_penguin, handle_update_penguin, handle_delete_penguin_by_id },
    AppState,
};

pub fn routes(app_state: Arc<AppState>) -> Router {

    //could improve on this
    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST])
    .allow_origin(Any);

    Router::new()
        .route("/hello", get(handle_hello))
        .route("/api/penguins", get(handle_get_penguins))
        .route("/api/penguins", post(handle_post_penguin))
        .route("/api/penguins/:id", put(handle_update_penguin))
        .route("/api/penguins/:id", get(handle_get_penguin_by_id))
        .route("/api/penguins/:id", delete(handle_delete_penguin_by_id))
        .layer(cors)
       .with_state(app_state)
}