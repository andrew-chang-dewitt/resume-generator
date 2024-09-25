use std::{io::Write, sync::Arc};

use async_trait::async_trait;
use clap::{Args, Subcommand};
use sqlx::{query, SqlitePool};

use crate::Models;

#[derive(Debug, Args)]
pub struct AddArgs {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Skill { name: String },
    Show { skill: Option<String> },
}

// pub async fn handle_add(
//     args: Cmds,
//     pool: SqlitePool,
//     writer: &mut impl Write,
// ) -> anyhow::Result<()> {
//     match args.cmd {
//         Some(Command::Skill { name }) => {
//             let id = models.skill.create(&name).await?;
//             writeln!(writer, "New Skill, {name}, created with id {id}")?;
//         }
//         Some(Command::Show { skill }) => {
//             if let Some(s) = skill {
//                 let results = Skill::
//             }
//         }
//         None => todo!(),
//     };
//
//     Ok(())
// }
