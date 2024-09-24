use async_trait::async_trait;
use sqlx::SqlitePool;

use std::sync::Arc;

#[mockall::automock]
#[async_trait]
pub trait ResumeModel {
    async fn add_data(&self, description: String) -> anyhow::Result<i64>;
    async fn del_data(&self, id: i64) -> anyhow::Result<bool>;
    async fn lst_data(&self) -> anyhow::Result<()>;
}

pub struct Resume {
    pool: Arc<SqlitePool>,
}

impl Resume {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl ResumeModel for Resume {
    async fn add_data(&self, description: String) -> anyhow::Result<i64> {
        unimplemented!()
    }

    async fn del_data(&self, id: i64) -> anyhow::Result<bool> {
        unimplemented!()
    }

    async fn lst_data(&self) -> anyhow::Result<()> {
        unimplemented!()
    }
}
