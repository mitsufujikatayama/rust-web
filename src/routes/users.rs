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
        .route("/", get(users_handler))
        .route("/add", post(add_user_handler))
}

async fn users_handler(State(state): State<SharedState>) -> Html<String> {
    #[cfg(debug_assertions)]
    {
        state.tera.write().await.full_reload().unwrap();
    }

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
    Form(payload): Form<models::user::CreateUserInput>,
) -> impl IntoResponse {
    let result = models::user::create(&state.pool, payload).await;

    match result {
        Ok(_) => {
            info!("User data added successfully");
            Redirect::to("/users")
        }
        Err(e) => {
            error!("Failed to create user: {}", e);
            Redirect::to("/users")
        }
    }
}
