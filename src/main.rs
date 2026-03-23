#![allow(unused)]

use std::path::PathBuf;
use clap::Parser;
use std::io;

mod blog;
mod html;
mod load;

#[derive(Debug, Parser)]
struct Args
{
    path: PathBuf,
}

fn rapp() -> Result<(), load::LoadError>
{
    let Args { path } = Args::parse();

    let blogs         = load::load_from_dir(&path)?
        .collect::<Vec<_>>();

    html::write(path, blogs.iter())?;

    Ok(())
}

fn main() -> anyhow::Result<()>
{
    rapp()?;
    Ok(())
}
