use axum::{routing::get, extract::Extension, Router};
use shuttle_runtime::CustomError;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool, Executor};
use std::{fs, sync::Arc};

use crate::routes::route_all::routes;
mod routes;

pub struct AppState {
    db: Pool<Postgres>
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres()] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("./../db/schema.sql"))
    .await
    .map_err(CustomError::new)?;

    let app = routes(Arc::new(AppState { db: pool.clone() }));

    Ok(app.into())
}

/* 
#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repository = web::Data::new(film_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(film_repository)
                .configure(api_lib::health::service)
                .configure(
                    api_lib::films::service::<api_lib::film_repository::PostgresFilmRepository>,
                ),
        )
    };
*/