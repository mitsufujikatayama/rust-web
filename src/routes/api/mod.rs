use crate::state::SharedState;
use axum::Router;

mod sensor;
mod user;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .nest("/sensors", sensor::routes()) // -> /api/sensors
        .nest("/users", user::routes()) // -> /api/users
}
