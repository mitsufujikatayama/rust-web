use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
}

pub async fn fetch_all(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT
            id, username, email,
            DATE_FORMAT(created_at, '%Y-%m-%d %H:%i:%s') as created_at
        FROM users
        ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn create(pool: &MySqlPool, input: CreateUserInput) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (username, email) VALUES (?, ?)",
        input.username,
        input.email
    )
    .execute(pool)
    .await?;
    Ok(())
}
