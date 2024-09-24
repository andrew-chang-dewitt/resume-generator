use clap::{Args, Parser, Subcommand};
use sqlx::SqlitePool;

use std::{io::Write, sync::Arc};

// use crate::model::ResumeModel;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add(AddArgs),
}

#[derive(Debug, Args)]
struct AddArgs {
    #[command(subcommand)]
    cmd: Option<AddCommand>,
}

#[derive(Debug, Subcommand)]
enum AddCommand {
    Skill { name: String },
}

pub struct App {
    pub pool: Arc<SqlitePool>,
}

impl App {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    pub async fn run(&self, args: Cli, writer: &mut impl Write) -> anyhow::Result<()> {
        match args.cmd {
            Command::Add(add) => match add.cmd {
                Some(AddCommand::Skill { name }) => {}
                None => {}
            },
        }

        Ok(())
    }
}
