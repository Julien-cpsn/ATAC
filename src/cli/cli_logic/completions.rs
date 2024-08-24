use std::env;
use std::str::FromStr;

use anyhow::anyhow;
use clap::CommandFactory;
use clap_complete::{generate_to, Shell};

use crate::cli::args::{Args, ARGS};
use crate::cli::commands::completions::CompletionsCommand;

pub fn generate_completions(completions_command: &CompletionsCommand) -> anyhow::Result<()> {
    let shell: Shell = match Shell::from_str(&completions_command.shell) {
        Ok(shell) => shell,
        Err(error) => {
            return Err(anyhow!(error));
        }
    };

    let path = match &ARGS.directory {
        None => &env::current_dir()?,
        Some(path) => path
    };
    
    let mut command = Args::command();
    generate_to(shell, &mut command, "atac", path)?;

    println!("Completions file generated into \"{}\"", path.display());

    Ok(())
}