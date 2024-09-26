use std::io::Write;

use clap::{Args, Subcommand};

use crate::model;
use crate::store::{Add as AddStore, Store};

#[derive(Debug, Args)]
pub struct Add {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Resume { name: String },
    // Skill { name: String },
}

impl Add {
    pub async fn handle(self, store: &mut Store, writer: &mut impl Write) -> anyhow::Result<i64> {
        match self.cmd {
            Command::Resume { name } => {
                let resume = model::Resume::new(name);
                Ok(store.add(resume))
            } // Command::Skill { name } => {
              //     let model::Skill {
              //         name: new_name,
              //         id: new_id,
              //     } = model::Skill::create(pool, name).await?;
              //     writeln!(writer, "New Skill, {new_name}, created with id {new_id}")?;

              //     Ok(new_id)
              // }
        }
    }
}
