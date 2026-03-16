use std::fs::File;
use std::io::{stdout, BufReader, BufWriter};
use ypbank_parser::{read_from, write_to, Format};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Использование: convert <входной_файл> <входной_формат> <выходной_формат>");
        eprintln!("Форматы: csv, txt, bin");
        std::process::exit(1);
    }
    let input_file = &args[1];
    let input_format: Format = args[2].parse()?;
    let output_format: Format = args[3].parse()?;

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let transactions = read_from(reader, input_format)?;

    let writer = BufWriter::new(stdout().lock());
    write_to(&transactions, writer, output_format)?;
    Ok(())
}