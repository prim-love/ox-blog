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
    org: PathBuf,
    out: PathBuf,
}

fn rapp() -> Result<(), load::LoadError>
{
    let Args { org, out } = Args::parse();

    let blogs         = load::load_from_dir(org)?
        .collect::<Vec<_>>();

    html::write(out, blogs.iter())?;

    Ok(())
}

fn main() -> anyhow::Result<()>
{
    rapp()?;
    Ok(())
}
