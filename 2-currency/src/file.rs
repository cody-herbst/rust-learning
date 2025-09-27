use std::{env, fs};
use std::io::Write;
use std::path::Path;
use reqwest::blocking::get;
use crate::LINK;

pub fn download_file() -> anyhow::Result<String> {

    let mut dir: Option<String> = None;
    match env::current_dir() {
        Ok(path) => {
            dir = Some(path.into_os_string().into_string().unwrap());
        },
        Err(error) => {
            eprintln!("{}", error);
        }
    }

    let download_dir = Path::new(&dir.unwrap()).join("download");

    if !Path::new(&download_dir).exists() {
        fs::create_dir_all(&download_dir)?;
    }

    let response = get(LINK)?.bytes()?;

    let file = fs::File::create(download_dir.join("latest.xml"));
    match file {
        Ok(mut file) => {
            file.write_all(response.as_ref())?
        }
        Err(error) => {
            eprintln!("{}", error)
        }
    }

    Ok(String::from("so"))
}