use async_trait::async_trait;
use sqlx::{query_as, SqlitePool};

use super::Model;

#[derive(Debug)]
pub struct Skill {
    pub id: i64,
    pub name: String,
}

#[async_trait]
impl Model for Skill {
    type Output = Self;
    async fn create(pool: &SqlitePool, name: &String) -> anyhow::Result<Self::Output> {
        query_as!(
            Skill,
            "INSERT INTO Skill (name) Values ($1) RETURNING name, id;",
            name
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
