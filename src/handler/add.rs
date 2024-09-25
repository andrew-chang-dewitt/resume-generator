use std::io::Write;

use clap::{Args, Subcommand};
use sqlx::SqlitePool;

use crate::model::{self, Model};

#[derive(Debug, Args)]
pub struct Add {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Skill { name: String },
}

impl Add {
    pub async fn handle(&self, pool: &SqlitePool, writer: &mut impl Write) -> anyhow::Result<i64> {
        match &self.cmd {
            Command::Skill { name } => {
                let model::Skill {
                    name: new_name,
                    id: new_id,
                } = model::Skill::create(pool, name).await?;
                writeln!(writer, "New Skill, {new_name}, created with id {new_id}")?;

                Ok(new_id)
            }
        }
    }
}
