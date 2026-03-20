use std::fs::File;
use std::io::BufReader;
use ypbank_parser::{read_from, Format};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("Использование: compare <файл1> <формат1> <файл2> <формат2>");
        std::process::exit(1);
    }
    let file1 = &args[1];
    let format1: Format = args[2].parse()?;
    let file2 = &args[3];
    let format2: Format = args[4].parse()?;

    let f1 = File::open(file1)?;
    let r1 = BufReader::new(f1);
    let txs1 = read_from(r1, format1)?;

    let f2 = File::open(file2)?;
    let r2 = BufReader::new(f2);
    let txs2 = read_from(r2, format2)?;

    if txs1 == txs2 {
        println!("Файлы идентичны");
    } else {
        println!("Файлы различаются");
    }
    Ok(())
}
