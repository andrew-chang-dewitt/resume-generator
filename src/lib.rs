use std::{env, io::Write};

use clap::{Parser, Subcommand, ValueEnum};
use log::{debug, info};
use sqlx::SqlitePool;
use store::Store;

mod changelist;
mod handler;
mod interaction;
mod logging;
mod model;
mod router;
mod state;
mod store;

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
    /// sqlite url connection string, can set via DATABASE_URL environment variable as well
    dburl: Option<String>,
    /// set output verbosity
    #[arg(short, long, value_enum)]
    verbose: Option<Verbosity>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(handler::Add),
    // Show(handler::Show),
}

/// Obj for holding active db pool, cli command given on exec, and necessary configuration
#[derive(Debug)]
pub struct App {
    cmd: Command,
    config: AppConfig,
    store: Store,
}

impl App {
    /// Create an application instance from parsed arguments
    pub async fn new(args: Args) -> anyhow::Result<Self> {
        // save command for later
        let cmd = args.cmd;
        // create config obj from args
        let config = AppConfig {
            dburl: match args.dburl {
                Some(s) => s,
                None => env::var("DATABASE_URL")?,
            },
            verbose: match args.verbose {
                Some(v) => v,
                None => Verbosity::Error,
            },
        };
        // setup logging
        logging::initialize(&config.verbose)?;
        info!("Logging initalized.");
        // connect to db
        let pool = SqlitePool::connect(&config.dburl).await?;
        debug!("DB pool connected.");
        // make sure db is up to date
        sqlx::migrate!().run(&pool).await?;
        debug!("DB schema up to date.");
        // init data store
        let store = Store::new(pool);

        Ok(Self { cmd, config, store })
    }

    /// Run app w/ command parsed from args & attach output to given write stream
    pub async fn run(mut self, writer: &mut impl Write) -> anyhow::Result<()> {
        debug!("Executing command {:#?}.", self.cmd);
        match self.cmd {
            Command::Add(add) => add.handle(&mut self.store, writer).await.map(|_| ()),
            // Command::Show(show) => show.handle(&self.pool, writer).await,
        }
    }
}

/// Config object for App
#[derive(Debug)]
pub struct AppConfig {
    /// Sqlite connection string, format `sqlite:<filepath>`
    dburl: String,
    /// Adjust output verbosity, defaults to only output errors
    verbose: Verbosity,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Verbosity {
    Debug,
    Info,
    Warn,
    Error,
}

// TODO: maybe something like this'll be useful to describe command handlers?
// #[async_trait]
// pub trait HandleCmd<'a, R> {
//     async fn handle(&self, pool: &'a SqlitePool, writer: &'a mut impl Write) -> anyhow::Result<R>;
// }
