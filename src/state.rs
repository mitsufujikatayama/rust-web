use sqlx::mysql::MySqlPool;
use std::sync::Arc;
use tera::Tera;
use tokio::sync::RwLock;

// アプリケーション全体で共有する状態
pub struct AppState {
    pub tera: RwLock<Tera>,
    pub pool: MySqlPool,
}

pub type SharedState = Arc<AppState>;
