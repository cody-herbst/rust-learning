use std::{env, fs};
use std::io::Write;
use std::path::Path;
use chrono::{DateTime, Local};
use reqwest::blocking::get;
use anyhow::Result;
use crate::LINK;

pub fn download_file() -> Result<String> {

    let download_dir = &"download";
    let file_name = &"latest.xml";

    let mut dir: Option<String> = None;
    match env::current_dir() {
        Ok(path) => {
            dir = Some(path.into_os_string().into_string().unwrap());
        },
        Err(error) => {
            eprintln!("{}", error);
        }
    }

    // check if the file is up to date
    let file = Path::new(&dir.unwrap()).join(download_dir).join(file_name);
    if fs::exists(&file)? {
        let time = fs::metadata(&file)?.modified()?;
        let date : DateTime<Local> = time.into();
        if date.date_naive() == Local::now().date_naive() {
            return Ok(fs::read_to_string(file)?)
        }
    }

    // file doesn't exist or is out of date
    let response = get(LINK)?.bytes()?;
    fs::create_dir_all(download_dir)?;
    let file = fs::File::create(&file);
    match file {
        Ok(mut file) => {
            file.write_all(response.as_ref())?
        }
        Err(error) => {
            eprintln!("{}", error)
        }
    }

     Ok(String::from_utf8(response.to_vec())?)
}