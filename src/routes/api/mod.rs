use axum::Router;
use crate::state::SharedState;

// サブモジュールの登録
mod sensor;
mod user;

// API全体のルーターを構築して返す
pub fn routes() -> Router<SharedState> {
    Router::new()
        // "/api" の下に何を生やすかここで定義
        .nest("/sensors", sensor::routes()) // -> /api/sensors
        .nest("/users", user::routes())     // -> /api/users
}