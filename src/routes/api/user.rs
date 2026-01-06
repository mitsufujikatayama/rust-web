use axum::{
    extract::State,
    routing::get,
    Json, Router,
    http::StatusCode,
};
use serde_json::{json, Value};
use crate::state::SharedState;
use crate::models;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/",
            get(list_json)
            .post(create_json)
        )
}

async fn list_json(State(state): State<SharedState>) -> Json<Vec<models::user::User>> {
    let users = models::user::fetch_all(&state.pool).await.unwrap_or_else(|_| vec![]);
    Json(users)
}

async fn create_json(
    State(state): State<SharedState>,
    Json(payload): Json<models::user::CreateUserInput>,
) -> (StatusCode, Json<Value>) {
    let result = models::user::create(&state.pool, payload).await;
    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "status": "success", "message": "User created." }))),
        Err(e) => {
            tracing::error!("API Error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "status": "error", "message": "Database error." })))
        }
    }
}