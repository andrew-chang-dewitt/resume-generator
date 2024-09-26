use std::io::Write;

use clap::{Args, Subcommand};
use log::debug;
use sqlx::SqlitePool;

mod one;

use one::One;

#[derive(Debug, Args)]
pub struct Show {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    One(One),
    // All(All),
}

impl Show {
    pub async fn handle(&self, pool: &SqlitePool, writer: &mut impl Write) -> anyhow::Result<()> {
        debug!("In show handler w/: {:#?}", &self.cmd);
        match &self.cmd {
            Command::One(one) => one.handle(pool, writer).await,
            // Command::All(all) => all.handle(pool, writer).await,
        }
    }
}
