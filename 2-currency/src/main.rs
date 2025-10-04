mod file;


use std::io::{ Write};
use std::process::exit;


const LINK: &str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";

enum Currency {
    USD,
    GBP,
    JPY,
    KRW
}

fn main() {
    println!("Hello, world!");

    match file::download_file() {
        Ok(currency) => {println!("file downloaded: {:?}", currency)},
        Err(err) => {eprintln!("{}", err); exit(1);}
    }
}


