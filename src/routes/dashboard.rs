use axum::{
    extract::{State, Form},
    response::{Html, Redirect, IntoResponse},
    routing::{get, post},
    Router,
};
use tera::Context;
use crate::state::SharedState;
// 【重要】モデル層を読み込み
use crate::models;
use tracing::{info, error};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", get(root_handler))
        .route("/add", post(add_sensor_data))
}

async fn root_handler(State(state): State<SharedState>) -> Html<String> {
    { state.tera.write().await.full_reload().unwrap(); }

    // 【修正】階層: models::sensor, 関数名: fetch_recent
    let rows = models::sensor::fetch_recent(&state.pool)
        .await
        .unwrap_or_else(|_| vec![]);

    let mut context = Context::new();
    context.insert("title", "至高のCMS");
    context.insert("content", "ダッシュボードへようこそ。");
    context.insert("sensors", &rows);

    let tera = state.tera.read().await;
    let rendered = tera.render("dashboard.html.tera", &context).unwrap();
    Html(rendered)
}

async fn add_sensor_data(
    State(state): State<SharedState>,
    // 【修正】階層: models::sensor
    Form(payload): Form<models::sensor::CreateSensorInput>,
) -> impl IntoResponse {
    // 【修正】階層: models::sensor, 関数名: create
    let result = models::sensor::create(&state.pool, payload).await;

    match result {
        Ok(_) => {
            info!("Sensor data added successfully");
            Redirect::to("/")
        },
        Err(e) => {
            error!("Failed to insert sensor data: {}", e);
            Redirect::to("/")
        }
    }
}