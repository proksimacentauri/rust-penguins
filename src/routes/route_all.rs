use std::sync::Arc;

use axum::{Router, routing::{get, post}, extract::State};
use crate::AppState;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn version(State(data): State<Arc<AppState>>) -> String {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),        
    }
}

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/version", get(version))
        .route("/", get(hello_world))
         .with_state(app_state)
}