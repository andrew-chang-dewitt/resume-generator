pub use Skill;

#[async_trait]
trait Model {
    async fn create(&self, name: &String) -> anyhow::Result<i64>;
}
