// use std::{io::Write, sync::Arc};

use std::{env, io::Write};

use clap::{Parser, Subcommand};
use sqlx::SqlitePool;

mod add;
// mod model;

#[derive(Debug, Parser)]
/// A Resume data storage & generation tool.
///
/// A CLI for organizing skills, education, jobs, projects, & other resume details, then
/// assists creating a resume file of various formats by interactively selecting data points to
/// include, then generating a file of the desired type.
pub struct Args {
    #[command(subcommand)]
    cmd: Command,
    #[arg(short, long)]
    /// sqlite url connection string, can set via DBURL environment variable as well
    dburl: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(add::AddArgs),
}

/// Obj for holding active db pool, cli command given on exec, and necessary configuration
pub struct App {
    cmd: Command,
    config: AppConfig,
    pool: SqlitePool,
}

impl App {
    /// Create an application instance from parsed arguments
    pub async fn new(args: Args) -> anyhow::Result<Self> {
        // create config obj from args
        let dburl = match args.dburl {
            Some(s) => s,
            None => env::var("DBURL")?,
        };
        let config = AppConfig::new(dburl);
        // connect to db
        let pool = SqlitePool::connect(&config.dburl).await?;
        // make sure db is up to date
        sqlx::migrate!().run(&pool).await?;

        Ok(Self {
            cmd: args.cmd,
            config,
            pool,
        })
    }

    /// Run app w/ command parsed from args & attach output to given write stream
    pub async fn run(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        todo!()
    }
}

/// Config object for App
pub struct AppConfig {
    /// Sqlite connection string, format `sqlite:<filepath>`
    dburl: String,
}

impl AppConfig {
    /// Build config object from given data
    fn new(dburl: String) -> Self {
        Self { dburl }
    }
}
