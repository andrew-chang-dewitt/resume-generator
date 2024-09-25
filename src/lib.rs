// use std::{io::Write, sync::Arc};

use std::{env, io::Write};

use clap::{Args, Parser, Subcommand};
use sqlx::SqlitePool;

// mod add;
// mod model;

// use crate::model::ResumeModel;

pub struct App<'a, Writer: Write> {
    cmd: Command,
    config: AppConfig,
    pool: SqlitePool,
    writer: Option<&'a mut Writer>,
}

impl<'a, Writer: Write> App<'a, Writer> {
    pub async fn new(args: AppArgs) -> anyhow::Result<Self> {
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
            writer: None,
        })
    }

    pub async fn run(&self, writer: &'a mut Writer) -> anyhow::Result<()> {
        todo!()
    }
}

pub struct AppConfig {
    dburl: String,
}

impl AppConfig {
    fn new(dburl: String) -> Self {
        Self { dburl }
    }
}

#[derive(Debug, Parser)]
/// A Resume data storage & generation tool.
///
/// A CLI for organizing skills, education, jobs, projects, & other resume details, then
/// assists creating a resume file of various formats by interactively selecting data points to
/// include, then generating a file of the desired type.
pub struct AppArgs {
    #[command(subcommand)]
    cmd: Command,
    #[arg(short, long)]
    /// sqlite url connection string, can set via DBURL environment variable as well
    dburl: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(AddArgs),
}

#[derive(Debug, Args)]
pub struct AddArgs {
    #[command(subcommand)]
    cmd: AddCommand,
}

#[derive(Debug, Subcommand)]
enum AddCommand {
    Skill { name: String },
    Show { skill: Option<String> },
}
