use std::env;

use sqlx::PgPool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // get env vars
    dotenvy::dotenv().ok();
    // create connection pool
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    // let's just start by printing some data:
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("{row:#?}");

    Ok(())
}
