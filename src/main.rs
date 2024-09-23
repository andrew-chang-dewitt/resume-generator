use sqlx::postgres::PgPoolOptions;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://andrew:pass@localhost:5432/resume")
        .await?;

    // let's just start by printing some data:
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("{row:#?}");

    Ok(())
}
