use async_trait::async_trait;
use sqlx::SqlitePool;

pub mod skill;
pub use skill::Skill;

#[async_trait]
pub trait Model {
    type Output;
    async fn create(pool: &SqlitePool, name: &String) -> anyhow::Result<Self::Output>;
}

// trait Id<T: Clone> {
//     fn id_field(&self) -> &T;
//
//     fn borrow_id(&self) -> &T {
//         self.id_field()
//     }
//     fn clone_id(&self) -> T {
//         let id = self.id_field();
//         id.clone()
//     }
// }
