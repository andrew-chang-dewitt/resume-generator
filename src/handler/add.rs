use std::io::Write;

use clap::{Args, Subcommand};

use crate::model;
use crate::state::{AddNew, Key};
use crate::store::Store;

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
    pub async fn handle(self, store: &mut Store, writer: &mut impl Write) -> anyhow::Result<Key> {
        match self.cmd {
            Command::Resume { name } => {
                let resume = model::Resume::new(name);
                Ok(store.add_new(resume))
            } // Command::Skill { name } => {
              //     let model::Skill {
              //         name: new_name,
              //         id: new_id,
              //     } = model::Skill::add_new(pool, name).await?;
              //     writeln!(writer, "New Skill, {new_name}, created with id {new_id}")?;

              //     Ok(new_id)
              // }
        }
    }
}
