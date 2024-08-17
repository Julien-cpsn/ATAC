use std::fs;
use clap::CommandFactory;

use crate::cli::args::{Args, ARGS};
use crate::cli::commands::man::ManCommand;

pub fn generate_man_page(man_command: &ManCommand) -> anyhow::Result<()> {

    let man = clap_mangen::Man::new(Args::command());
    let mut buffer: Vec<u8> = vec![];

    man.render(&mut buffer)?;

    let path = match &man_command.output_directory {
        None => &ARGS.directory,
        Some(path) => path
    };
    
    fs::write(path.join("atac.1"), buffer)?;
    
    println!("Man page generated into \"{}\"", path.display());

    Ok(())
}