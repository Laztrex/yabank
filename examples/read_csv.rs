mod common;

use std::fs::File;
use std::io::BufReader;
use ypbank_parser::{read_from, Format};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = common::records_csv_path();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let transactions = read_from(reader, Format::Csv)?;
    println!("Прочитано {} транзакций из CSV:", transactions.len());
    for tx in transactions {
        println!("{:?}", tx);
    }
    Ok(())
}