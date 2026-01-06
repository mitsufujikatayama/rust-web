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
        .route("/", get(users_handler))
        .route("/add", post(add_user_handler))
}

async fn users_handler(State(state): State<SharedState>) -> Html<String> {
    { state.tera.write().await.full_reload().unwrap(); }

    // 【修正】階層: models::user, 関数名: fetch_all
    let users = models::user::fetch_all(&state.pool)
        .await
        .unwrap_or_else(|e| {
            error!("DB Error: {}", e);
            vec![]
        });

    let mut context = Context::new();
    context.insert("users", &users);

    let tera = state.tera.read().await;
    let rendered = tera.render("users.html.tera", &context).unwrap();
    Html(rendered)
}

async fn add_user_handler(
    State(state): State<SharedState>,
    // 【修正】階層: models::user
    Form(payload): Form<models::user::CreateUserInput>,
) -> impl IntoResponse {
    // 【修正】階層: models::user, 関数名: create
    let result = models::user::create(&state.pool, payload).await;

    match result {
        Ok(_) => {
            info!("User data added successfully");
            Redirect::to("/users")
        },
        Err(e) => {
            error!("Failed to create user: {}", e);
            Redirect::to("/users")
        }
    }
}