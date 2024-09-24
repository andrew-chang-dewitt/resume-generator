use std::env;

use sqlx::PgPool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // get env vars
    dotenvy::dotenv().ok();
    // create connection pool
    let user = &env::var("PGUSER")?;
    let pass = &env::var("PGPASS")?;
    let host = &env::var("PGHOST")?;
    let port = &env::var("PGPORT")?;
    let name = &env::var("DBNAME")?;
    let url = format!("postgres://{user}:{pass}@{host}:{port}/{name}");
    let pool = PgPool::connect(&url).await?;

    // let's just start by printing some data:
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("{row:#?}");

    Ok(())
}
