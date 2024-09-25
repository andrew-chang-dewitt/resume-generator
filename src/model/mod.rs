use async_trait::async_trait;
use sqlx::SqlitePool;

pub mod skill;
pub use skill::Skill;

#[async_trait]
pub trait Model<T: Clone>: Id<T> {
    type Output;
    async fn create(pool: &SqlitePool, name: &String) -> anyhow::Result<Self::Output>;
    async fn get_one_by_id(pool: &SqlitePool, id: &T) -> anyhow::Result<Option<Self::Output>>;
    async fn get_one_by_field(
        pool: &SqlitePool,
        field: &String,
        value: &String,
    ) -> anyhow::Result<Option<Self::Output>>;
    async fn get_one_like_field(
        pool: &SqlitePool,
        field: &String,
        query: &String,
    ) -> anyhow::Result<Option<Self::Output>>;
}

pub trait Id<T: Clone> {
    fn id_field(&self) -> &T;

    fn borrow_id(&self) -> &T {
        self.id_field()
    }
    fn clone_id(&self) -> T {
        let id = self.id_field();
        id.clone()
    }
}
