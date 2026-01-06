use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

// サブモジュールの公開
pub mod sensor;
pub mod user;

// 共通のDB初期化ロジック
pub async fn init_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .after_connect(|conn, _meta| Box::pin(async move {
            sqlx::query("SET time_zone = '+09:00';")
                .execute(conn)
                .await?;
            Ok(())
        }))
        .connect(database_url)
        .await
}