use axum::{
    extract::State,
    routing::get,
    Json, Router,
    http::StatusCode,
};
use serde_json::{json, Value};
use crate::state::SharedState;
use crate::models;

// サブ・ルーター
pub fn routes() -> Router<SharedState> {
    Router::new()
        // 親が "/api/sensors" に割り当てるので、ここはルート "/" でOK
        .route("/",
            get(list_json)
            .post(create_json)
        )
}

async fn list_json(State(state): State<SharedState>) -> Json<Vec<models::sensor::SensorData>> {
    let rows = models::sensor::fetch_recent(&state.pool).await.unwrap_or_else(|_| vec![]);
    Json(rows)
}

async fn create_json(
    State(state): State<SharedState>,
    Json(payload): Json<models::sensor::CreateSensorInput>,
) -> (StatusCode, Json<Value>) {
    let result = models::sensor::create(&state.pool, payload).await;
    match result {
        Ok(_) => (StatusCode::CREATED, Json(json!({ "status": "success", "message": "Sensor data created." }))),
        Err(e) => {
            tracing::error!("API Error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "status": "error", "message": "Database error." })))
        }
    }
}