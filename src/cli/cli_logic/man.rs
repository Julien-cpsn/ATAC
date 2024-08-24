use std::env;
use std::fs;
use clap::CommandFactory;

use crate::cli::args::{Args, ARGS};

pub fn generate_man_page() -> anyhow::Result<()> {

    let man = clap_mangen::Man::new(Args::command());
    let mut buffer: Vec<u8> = vec![];

    man.render(&mut buffer)?;

    let path = match &ARGS.directory {
        None => &env::current_dir()?,
        Some(path) => path
    };
    
    fs::write(path.join("atac.1"), buffer)?;
    
    println!("Man page generated into \"{}\"", path.display());

    Ok(())
}