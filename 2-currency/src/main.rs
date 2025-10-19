#![feature(slice_as_array)]
mod file;

use std::io;
use std::process::exit;
use std::str::FromStr;
use strum::VariantNames;
use strum_macros::{EnumString, VariantNames};

const LINK: &str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";


#[derive(VariantNames, EnumString)]
enum Currency {
    USD,
    GBP,
    JPY,
    KRW
}

fn main() {
    println!("Hello, world!");

    let xml = match file::download_file() {
        Ok(currency) => {
            println!("file downloaded: {:?}", currency);
            currency
        },
        Err(err) => {eprintln!("{}", err); exit(1);}
    };

    handle_conversion(xml);
}

fn handle_conversion(xml: String) {

    println!("Pick a origin currency: {:?}", Currency::VARIANTS);
    let mut original_currency_input = String::new();
    io::stdin().read_line(&mut original_currency_input).unwrap();
    let origin = Currency::from_str(original_currency_input.as_str()).unwrap();

    println!("Pick a number: ");
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input).unwrap();

    println!("Pick a target currency: ");
    let mut target_currency_input = String::new();
    io::stdin().read_line(&mut target_currency_input).unwrap();
    let target = Currency::from_str(target_currency_input.as_str()).unwrap();


}


