use async_trait::async_trait;
use clap::{Parser, Subcommand};
use sqlx::SqlitePool;

use std::{env, io::Write, sync::Arc};

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
    let dburl = &env::var("DBURL")?;
    let pool = SqlitePool::connect(&dburl).await?;
    // and create data model w/ db connection
    let resume = Resume::new(pool);

    // setup stdio write stream
    let mut writer = std::io::stdout();

    handle_command(args, resume, &mut writer).await
}

async fn handle_command(
    args: Args,
    resume: impl ResumeModel,
    writer: &mut impl Write,
) -> anyhow::Result<()> {
    match args.cmd {
        Some(Command::Add { description }) => {
            writeln!(writer, "Adding data w/ description '{}'", &description)?;
            let id = resume.add_data(description).await?;
            writeln!(writer, "Added data with id {id}")?;
        }
        Some(Command::Del { id }) => {
            writeln!(writer, "Deleting data w/ id '{}'", &id)?;
            if resume.del_data(id).await? {
                writeln!(writer, "Deleted data with id {id}")?;
            } else {
                writeln!(writer, "Invalid id {id}")?;
            }
        }
        None => {
            writeln!(writer, "Printing all data")?;
            resume.lst_data().await?;
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

struct Resume {
    pool: Arc<SqlitePool>,
}

impl Resume {
    fn new(pool: SqlitePool) -> Self {
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
