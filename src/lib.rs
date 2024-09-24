use std::io::Write;

use clap::{Parser, Subcommand};
use sqlx::SqlitePool;

mod add;

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

pub async fn run(args: Cli, pool: SqlitePool, writer: &mut impl Write) -> anyhow::Result<()> {
    match args.cmd {
        Command::Add(add) => add::handle_add(add, pool, writer).await?,
    };

    Ok(())
}
