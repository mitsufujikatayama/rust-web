use sqlx::mysql::MySqlPool;
use serde::{Deserialize, Serialize};

// 構造体定義
#[derive(sqlx::FromRow, Serialize)]
pub struct SensorData {
    pub id: i32,
    pub temperature: f64,
    pub heart_rate: i32,
    pub recorded_at: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateSensorInput {
    pub temperature: f64,
    pub heart_rate: i32,
}

// ロジック実装
pub async fn fetch_recent(pool: &MySqlPool) -> Result<Vec<SensorData>, sqlx::Error> {
    sqlx::query_as!(
        SensorData,
        r#"
        SELECT
            id, temperature, heart_rate,
            DATE_FORMAT(recorded_at, '%Y-%m-%d %H:%i:%s') as recorded_at
        FROM sensor_data
        ORDER BY id DESC LIMIT 5
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn create(pool: &MySqlPool, input: CreateSensorInput) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO sensor_data (temperature, heart_rate) VALUES (?, ?)",
        input.temperature,
        input.heart_rate
    )
    .execute(pool)
    .await?;
    Ok(())
}