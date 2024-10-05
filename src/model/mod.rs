use async_trait::async_trait;
use sqlx::SqlitePool;

// pub mod skill;
// pub use skill::Skill;

//FIXME: maybe this should all be impl on the enum?
#[async_trait]
pub trait ModelUnused {
    // pub trait Model<T: Clone>: Id<T> {
    type Output;
    async fn create(pool: &SqlitePool, name: &String) -> anyhow::Result<Self::Output>;
    // async fn get_one_by_id(pool: &SqlitePool, id: &T) -> anyhow::Result<Self::Output>;
    // async fn get_one_includes(pool: &SqlitePool, query: &String) -> anyhow::Result<Self::Output>;
    // async fn get_all(pool: &SqlitePool) -> anyhow::Result<Vec<Self::Output>>;
}

// pub trait Id<T: Clone> {
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

#[derive(Debug)]
pub enum Model {
    Resume(Resume),
}

#[derive(Debug)]
pub enum ModelKey<KeyType> {
    Resume(KeyType),
}

#[derive(Debug)]
pub struct Resume {
    name: String,
}

impl Resume {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
