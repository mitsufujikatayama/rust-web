use crate::models;
use crate::state::SharedState;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use tera::Context;
use tracing::{error, info};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(root_handler))
        .route("/add", post(add_sensor_data))
}

async fn root_handler(State(state): State<SharedState>) -> Html<String> {
    #[cfg(debug_assertions)]
    {
        state.tera.write().await.full_reload().unwrap();
    }

    let rows = models::sensor::fetch_recent(&state.pool)
        .await
        .unwrap_or_else(|_| vec![]);

    let mut context = Context::new();
    context.insert("title", "RustのCMS");
    context.insert("content", "ダッシュボードへようこそ。");
    context.insert("sensors", &rows);

    let tera = state.tera.read().await;
    let rendered = tera.render("dashboard.html.tera", &context).unwrap();
    Html(rendered)
}

async fn add_sensor_data(
    State(state): State<SharedState>,
    Form(payload): Form<models::sensor::CreateSensorInput>,
) -> impl IntoResponse {
    let result = models::sensor::create(&state.pool, payload).await;

    match result {
        Ok(_) => {
            info!("Sensor data added successfully");
            Redirect::to("/")
        }
        Err(e) => {
            error!("Failed to insert sensor data: {}", e);
            Redirect::to("/")
        }
    }
}
