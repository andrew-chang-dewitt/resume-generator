use std::{io::Write, sync::Arc};

use clap::{Parser, Subcommand};
use sqlx::SqlitePool;

mod add;
mod model;

// use crate::model::ResumeModel;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add(add::Cmds),
}

struct Models {
    pool: Arc<SqlitePool>,
    skill: Option<add::Skill>,
}

impl Models {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
            skill: None,
        }
    }

    pub fn init_skill(&self) {
        self.skill = Some(add::Skill::new(pool));
    }
}

pub async fn run(args: Cli, pool: SqlitePool, writer: &mut impl Write) -> anyhow::Result<()> {
    match args.cmd {
        Command::Add(add) => add::handle_add(add, pool, writer).await?,
    };

    Ok(())
}
