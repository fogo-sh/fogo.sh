use crate::parser::parse;
use crate::structs::{Author, GithubAuthor, Post};

use crate::builder::{Asset, CopyFile};
use crate::error::{Error, Result};
use std::fs;
use std::fs::DirEntry;
use std::io;

fn fetch_user(author: &mut Author) -> Result<GithubAuthor> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Site/1.0.0 (https://github.com/fogo-sh/fogo.sh)")
        .build()?;
    let response = client
        .get(format!(
            "https://api.github.com/users/{}",
            author.meta.username
        ))
        .send()?;

    let response_text = &response.text()?;
    Ok(serde_json::from_str(response_text)?)
}

fn load_and_parse_post(dir_entry: std::io::Result<DirEntry>) -> Result<Post> {
    let entry = dir_entry?;
    if entry.file_type()?.is_dir() {
        return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
    }
    parse(&fs::read_to_string(&entry.path())?).map_err(Error::Toml)
}

fn load_and_parse_author(dir_entry: std::io::Result<DirEntry>) -> Result<Author> {
    let entry = dir_entry?;
    if entry.file_type()?.is_dir() {
        return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
    }
    parse(&fs::read_to_string(&entry.path())?).map_err(Error::Toml)
}

fn collect_others(dir_entry: std::io::Result<DirEntry>) -> Result<Asset> {
    let entry = dir_entry?;
    if entry.file_type()?.is_dir() {
        return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
    }
    let path = entry.path();
    Ok(Asset::Other(CopyFile { path }))
}

pub fn fetch_posts() -> Result<Vec<Post>> {
    fs::read_dir("./posts")?.map(load_and_parse_post).collect()
}

pub fn fetch_authors() -> Result<Vec<Author>> {
    let mut authors = fs::read_dir("./authors")?
        .map(load_and_parse_author)
        .collect::<Result<Vec<Author>>>()?;
    for mut author in &mut authors {
        author.meta.github = Some(fetch_user(&mut author)?);
    }
    Ok(authors)
}

pub fn fetch_assets() -> Result<Vec<Asset>> {
    fs::read_dir("./assets")?.map(collect_others).collect()
}
