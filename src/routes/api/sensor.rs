use crate::models;
use crate::state::SharedState;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(list_json).post(create_json))
}

async fn list_json(State(state): State<SharedState>) -> Json<Vec<models::sensor::SensorData>> {
    let rows = models::sensor::fetch_recent(&state.pool)
        .await
        .unwrap_or_else(|_| vec![]);
    Json(rows)
}

async fn create_json(
    State(state): State<SharedState>,
    Json(payload): Json<models::sensor::CreateSensorInput>,
) -> (StatusCode, Json<Value>) {
    let result = models::sensor::create(&state.pool, payload).await;
    match result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "status": "success", "message": "Sensor data created." })),
        ),
        Err(e) => {
            tracing::error!("API Error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": "Database error." })),
            )
        }
    }
}
