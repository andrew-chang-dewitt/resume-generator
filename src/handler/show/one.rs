use std::io::Write;

use clap::{Args, Subcommand};
use sqlx::SqlitePool;

use crate::model::{self, Model};

#[derive(Debug, Args)]
pub struct One {
    #[command(subcommand)]
    cmd: OneCommand,
}

#[derive(Debug, Subcommand)]
enum OneCommand {
    Skill(Skill),
}

impl One {
    pub async fn handle(&self, pool: &SqlitePool, writer: &mut impl Write) -> anyhow::Result<()> {
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
    Includes { query: String },
}

impl Skill {
    pub async fn handle(
        &self,
        pool: &SqlitePool,
        writer: &mut impl Write,
    ) -> anyhow::Result<model::Skill> {
        let skill = match &self.cmd {
            SkillCommand::ById { id } => model::Skill::get_one_by_id(pool, id).await?,
            SkillCommand::Includes { query } => model::Skill::get_one_includes(pool, query).await?,
        };

        writeln!(
            writer,
            "Skill found: {id}: {name}",
            id = skill.id,
            name = skill.name
        )?;

        Ok(skill)
    }
}
