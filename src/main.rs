use async_trait::async_trait;
use clap::{Parser, Subcommand};
use sqlx::PgPool;

use std::{env, io::Write};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add { description: String },
    Del { id: i64 },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // get env vars
    dotenvy::dotenv().ok();

    // get cli args
    let args = Args::parse();

    // create db connection
    let user = &env::var("PGUSER")?;
    let pass = &env::var("PGPASS")?;
    let host = &env::var("PGHOST")?;
    let port = &env::var("PGPORT")?;
    let name = &env::var("DBNAME")?;
    let url = format!("postgres://{user}:{pass}@{host}:{port}/{name}");
    let pool = PgPool::connect(&url).await?;
    // and create data model w/ db connection
    let resume_model = ResumeModel::new(pool);

    // setup stdio write stream
    let mut writer = std::io::stdout();

    handle_command(args, model, &mut writer).await
}

async fn handle_command(
    args: Args,
    model: impl ResumeModel,
    writer: &mut impl Write,
) -> anyhow::Result<()> {
    match args.cmd {
        Some(Command::Add { description }) => {
            writeln!(writer, "Adding data w/ description '{}'", &description)?;
            let id = model.add_data(description).await?;
            writeln!(writer, "Added data with id {id}")?;
        }
        Some(Command::Del { id }) => {
            writeln!(writer, "Deleting data w/ id '{}'", &id)?;
            if model.del_data(id).await? {
                writeln!(writer, "Deleted data with id {id}")?;
            } else {
                writeln!(writer, "Invalid id {id}")?;
            }
        }
        None => {
            writeln!(writer, "Printing all data")?;
            model.lst_data().await?;
        }
    }

    Ok(())
}

#[async_trait]
trait ResumeModel {
    async fn add_data(&self, description: String) -> anyhow::Result<i64>;
    async fn del_data(&self, id: i64) -> anyhow::Result<bool>;
    async fn lst_data(&self) -> anyhow::Result<()>;
}
