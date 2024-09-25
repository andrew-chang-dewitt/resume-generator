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

    async fn get_one_by_id(pool: &SqlitePool, id: &i64) -> anyhow::Result<Option<Self::Output>> {
        query_as!(Skill, "SELECT * FROM Skill WHERE id=$1;", id,)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.into())
    }

    async fn get_one_by_field(
        pool: &SqlitePool,
        field: &String,
        value: &String,
    ) -> anyhow::Result<Option<Self::Output>> {
        todo!()
    }

    async fn get_one_like_field(
        pool: &SqlitePool,
        field: &String,
        query: &String,
    ) -> anyhow::Result<Option<Self::Output>> {
        todo!()
    }
}
