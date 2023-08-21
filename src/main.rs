use axum::http::Method;
use shuttle_runtime::CustomError;
use sqlx::{Postgres, Pool, Executor};
use std::sync::Arc;
use crate::routes::route_all::routes;
mod handlers;
mod models;
mod routes;
use axum_extra::routing::SpaRouter;
use tower_http::cors::{CorsLayer, Any};

pub struct AppState {
    db: Pool<Postgres>,
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,  #[shuttle_static_folder::StaticFolder] static_folder: std::path::PathBuf ) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("./../db/schema.sql"))
    .await
    .map_err(CustomError::new)?;
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    let app = routes(Arc::new(AppState { db: pool.clone()}))    
    .merge(SpaRouter::new("/", static_folder).index_file("index.html"));

    Ok(app.into())
}


