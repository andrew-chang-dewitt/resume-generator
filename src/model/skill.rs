use async_trait::async_trait;
use sqlx::{query_as, SqlitePool};

use super::{Id, Model};

#[derive(Debug)]
pub struct Skill {
    pub id: i64,
    pub name: String,
}

impl Id<i64> for Skill {
    fn id_field(&self) -> &i64 {
        &self.id
    }
}

#[async_trait]
impl Model<i64> for Skill {
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

    async fn get_one_by_id(pool: &SqlitePool, id: &i64) -> anyhow::Result<Self::Output> {
        query_as!(Skill, "SELECT * FROM Skill WHERE id=$1;", id)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    async fn get_one_includes(pool: &SqlitePool, query: &String) -> anyhow::Result<Self::Output> {
        let q = format!("%{query}%");
        query_as!(Skill, "SELECT * FROM Skill WHERE name LIKE $1", q)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    async fn get_all(pool: &SqlitePool) -> anyhow::Result<Vec<Self::Output>> {
        todo!()
    }
}
