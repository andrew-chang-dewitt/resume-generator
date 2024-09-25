use async_trait::async_trait;
use sqlx::{query, SqlitePool};

pub struct Skill;

impl Skill {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl Model for Skill {
    async fn create(&self, name: &String) -> anyhow::Result<i64> {
        let res = query!("INSERT INTO Skill (name) Values ($1) RETURNING id;", name)
            .fetch_one(&*self.pool)
            .await?;

        Ok(res.id)
    }
}
