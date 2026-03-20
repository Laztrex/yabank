mod common;

use std::env;
use std::fs::File;
use std::io::BufReader;
use ypbank_parser::{read_from, Format};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        args[1].clone().into()
    } else {
        common::records_txt_path()
    };
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let transactions = read_from(reader, Format::Txt)?;
    println!("Прочитано {} транзакций из TXT:", transactions.len());
    for tx in transactions {
        println!("{:?}", tx);
    }
    Ok(())
}
