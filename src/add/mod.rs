use std::{io::Write, sync::Arc};

use async_trait::async_trait;
use clap::{Args, Subcommand};
use sqlx::{query, SqlitePool};

#[derive(Debug, Args)]
pub struct Cmds {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Skill { name: String },
}

pub async fn handle_add(
    args: Cmds,
    pool: SqlitePool,
    writer: &mut impl Write,
) -> anyhow::Result<()> {
    match args.cmd {
        Some(Command::Skill { name }) => {
            let id = Skill::new(pool).create(&name).await?;
            writeln!(writer, "New Skill, {name}, created with id {id}")?;
        }
        None => todo!(),
    };

    Ok(())
}

#[async_trait]
trait Model {
    async fn create(&self, name: &String) -> anyhow::Result<i64>;
}

#[derive(Debug)]
struct Skill {
    pool: Arc<SqlitePool>,
}

impl Skill {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl Model for Skill {
    async fn create(&self, name: &String) -> anyhow::Result<i64> {
        let res = query!("INSERT INTO Skill (name) Values ($1) RETURNING id;", name)
            .fetch_one(&*self.pool)
            .await?;

        Ok(res.id)
    }
}
