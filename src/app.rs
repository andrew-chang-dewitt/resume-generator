use clap::{Parser, Subcommand};

use std::io::Write;

use crate::model::ResumeModel;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Add { description: String },
    Del { id: i64 },
}

pub async fn handle_command(
    args: Args,
    resume: impl ResumeModel,
    writer: &mut impl Write,
) -> anyhow::Result<()> {
    match args.cmd {
        Some(Command::Add { description }) => {
            writeln!(writer, "Adding data w/ description '{}'", &description)?;
            let id = resume.add_data(description).await?;
            writeln!(writer, "Added data with id {id}")?;
        }
        Some(Command::Del { id }) => {
            writeln!(writer, "Deleting data w/ id '{}'", &id)?;
            if resume.del_data(id).await? {
                writeln!(writer, "Deleted data with id {id}")?;
            } else {
                writeln!(writer, "Invalid id {id}")?;
            }
        }
        None => {
            writeln!(writer, "Printing all data")?;
            resume.lst_data().await?;
        }
    }

    Ok(())
}
