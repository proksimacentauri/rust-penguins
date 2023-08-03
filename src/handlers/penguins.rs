use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};
use sqlx::query_as;
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    models::models::{CreatePenguin, UpdatePenguin, Penguin},
    AppState,
};

pub async fn handle_hello() -> &'static str {
    return "Hello, World!";
}

pub async fn handle_get_penguins (State(data): State<Arc<AppState>>) -> impl IntoResponse {
    const QUERY: &str = "SELECT id, penguin_name, age, created_at, updated_at FROM penguins";
    let penguins: Vec<Penguin> = query_as(QUERY).fetch_all(&data.db).await.unwrap();
    println!("\n=== select penguins with query.map...: \n{:?}", penguins);
    ((StatusCode::OK), Json(penguins))
}

pub async fn handle_post_penguin(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePenguin>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query = format!(
        "INSERT INTO penguins (penguin_name, age) VALUES ('{}', '{}') RETURNING *",
        body.penguin_name, body.age
    );

    let query_result = query_as::<_, Penguin>(&query)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(penguin) => {
            let penguin_response = json!({
                "status": "success",
                "data": {
                    "penguin": {
                        "id": penguin.id,
                        "penguin_name": penguin.penguin_name,
                        "age": penguin.age,
                    }
                }
            });
            return Ok((StatusCode::CREATED, Json(penguin_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}


pub async fn handle_update_penguin(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdatePenguin>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let updated_film = sqlx::query_as::<_, Penguin>(
        r#"
        UPDATE penguins
        SET penguin_name = $2, age = $3
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(body.penguin_name)
    .bind(body.age)
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());


    match updated_film {
        Ok(penguin) => {
            let penguin_response = json!({
                "status": "success",
                "data": {
                    "penguin": {
                        "id": penguin.id,
                        "penguin_name": penguin.penguin_name,
                        "age": penguin.age,
                    }
                }
            });
            return Ok((StatusCode::OK, Json(penguin_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn handle_get_penguin_by_id(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let penguin: Result<Penguin, String> = sqlx::query_as::<_, Penguin>(
        r#"
  SELECT id, penguin_name, age, created_at, updated_at
  FROM penguins
  WHERE id = $1
  "#,
    )
    .bind(id)
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

        match penguin {
            Ok(penguin) => {
                let penguin_response = json!({
                    "status": "success",
                    "data": {
                        "penguin": {
                            "id": penguin.id,
                            "penguin_name": penguin.penguin_name,
                            "age": penguin.age,
                        }
                    }
                });
                return Ok((StatusCode::OK, Json(penguin_response)));
            }
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"status": "error","message": format!("{:?}", e)})),
                ));
            }
        }
    }

    pub async fn handle_delete_penguin_by_id(
        Path(id): Path<Uuid>,
        State(data): State<Arc<AppState>>,
    ) -> impl IntoResponse {
        let result = sqlx::query_scalar::<_, Uuid>(
            r#"
            DELETE FROM penguins
            WHERE id = $1
            RETURNING id
        "#,
    )
    .bind(id)
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some(penguin)) => {
            let penguin_response = json!({
                "status": "success",
                "message": format!("Penguin with ID: {} was deleted", penguin)
            });
            (StatusCode::OK, Json(penguin_response))
        }
        Ok(None) => {
            // Record not found
            (StatusCode::NOT_FOUND, Json(json!({"status": "error", "message": "Penguin not found"})))
        }
        Err(e) => {
            // Handle or log the error as needed
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "error", "message": format!("{:?}", e)})))
        }
    }
}