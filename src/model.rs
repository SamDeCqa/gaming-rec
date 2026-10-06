use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct GameModel {
    pub id: Uuid,
    pub name: String,
    pub creator: String,
    pub plays: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UpdateGameModel {
    pub name: Option<String>,
    pub creator: Option<String>,
    pub plays: Option<i32>,
}
