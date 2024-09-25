use std::io::Write;

use anyhow::anyhow;
use clap::{Args, Subcommand};
use log::debug;
use sqlx::SqlitePool;

use crate::model::{self, Model};

#[derive(Debug, Args)]
pub struct Show {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    One(One),
}

impl Show {
    pub async fn handle(&self, pool: &SqlitePool, writer: &mut impl Write) -> anyhow::Result<()> {
        debug!("In show handler w/: {:#?}", &self.cmd);
        match &self.cmd {
            Command::One(one) => one.handle(pool, writer).await,
        }
    }
}

#[derive(Debug, Args)]
struct One {
    #[command(subcommand)]
    cmd: OneCommand,
}

#[derive(Debug, Subcommand)]
enum OneCommand {
    Skill(Skill),
}

impl One {
    pub async fn handle(&self, pool: &SqlitePool, writer: &mut impl Write) -> anyhow::Result<()> {
        debug!("In show one handler w/: {:#?}", &self.cmd);
        match &self.cmd {
            OneCommand::Skill(skill) => skill.handle(pool, writer).await.map(|_| ()),
        }
    }
}

#[derive(Debug, Args)]
struct Skill {
    #[command(subcommand)]
    cmd: SkillCommand,
}

#[derive(Debug, Subcommand)]
enum SkillCommand {
    ById { id: i64 },
    ByField { field: String, value: String },
    LikeField { field: String, query: String },
}

impl Skill {
    pub async fn handle(
        &self,
        pool: &SqlitePool,
        writer: &mut impl Write,
    ) -> anyhow::Result<model::Skill> {
        debug!("In show one skill handler w/: {:#?}", &self.cmd);
        let skill = match &self.cmd {
            SkillCommand::ById { id } => {
                if let Some(skill) = model::Skill::get_one_by_id(pool, id).await? {
                    Ok(skill)
                } else {
                    let err = anyhow!("No record found matching id {id:?}");
                    writeln!(writer, "{err:#?}")?;
                    Err(err)
                }
            }
            SkillCommand::ByField { field, value } => {
                // model::Skill::get_one_by_field(pool, field, value).await
                todo!()
            }
            SkillCommand::LikeField { field, query } => {
                // model::Skill::get_one_like_field(pool, field, query).await
                todo!()
            }
        };

        if let Ok(s) = &skill {
            writeln!(
                writer,
                "Skill found: {id}: {name}",
                id = s.id,
                name = s.name
            )?;
        };

        skill
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DataErr<T: std::fmt::Debug> {
    #[error("No record found matching id {id:?}")]
    NotFoundById { id: T },
}
