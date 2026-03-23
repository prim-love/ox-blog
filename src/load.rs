use crate::blog::{self, Blog};
use itertools::Itertools;
use thiserror::Error;
use walkdir::WalkDir;
use std::path::Path;
use orgize as org;
use std::io;

#[derive(Debug, Error)]
pub enum LoadError
{
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
    #[error(transparent)]
    Io(#[from] io::Error),}

pub fn load_from_file(path: impl AsRef<Path>) -> Result<impl Iterator<Item = Blog>, LoadError>
{
    let str = std::fs::read_to_string(path)?;
    let org = org::Org::parse(&str);
    let doc = org.document();

    Ok(blog::from_document(&doc))
}

pub fn load_from_dir(path: impl AsRef<Path>) -> Result<impl Iterator<Item = Blog>, LoadError>
{
    let dir: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| load_from_file(x.into_path()))
        .try_collect()?;

    Ok(dir.into_iter().flatten())
}
