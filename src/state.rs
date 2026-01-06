use std::sync::Arc;
use tokio::sync::RwLock;
use tera::Tera;
use sqlx::mysql::MySqlPool;

// アプリケーション全体で共有する状態
// main.rs だけでなく、他のモジュールからも参照されるため独立させます
pub struct AppState {
    pub tera: RwLock<Tera>,
    pub pool: MySqlPool,
}

// 便利なエイリアスを作っておくと楽です
pub type SharedState = Arc<AppState>;