use clap::Parser;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Read};
use ypbank_parser::{Format, read_from, write_to};

/// Cli - интерфейс конвертера файлов транзакций между форматами.
/// input: входной файл (опционально), если не задан - чтение из stdin
/// input_format: формат входного файла (csv, txt, bin)
/// output_format: формат выходного файла (csv, txt, bin)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    input: Option<String>,

    #[arg(long)]
    input_format: String,

    #[arg(long)]
    output_format: String,
}

fn main() {
    let cli = Cli::parse();

    let input_format = match cli.input_format.parse::<Format>() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Invalid input format '{}': {}", cli.input_format, e);
            eprintln!("Supported formats: csv, txt, bin");
            std::process::exit(1);
        }
    };
    let output_format = match cli.output_format.parse::<Format>() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Invalid output format '{}': {}", cli.output_format, e);
            eprintln!("Supported formats: csv, txt, bin");
            std::process::exit(1);
        }
    };

    let input: Box<dyn Read> = match cli.input.as_deref() {
        Some("-") | None => Box::new(stdin().lock()),
        Some(path) => match File::open(path) {
            Ok(f) => Box::new(f),
            Err(e) => {
                eprintln!("Failed to open input file '{}': {}", path, e);
                std::process::exit(1);
            }
        },
    };
    let reader = BufReader::new(input);

    let transactions = match read_from(reader, input_format) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            std::process::exit(1);
        }
    };

    let writer = BufWriter::new(stdout().lock());
    if let Err(e) = write_to(&transactions, writer, output_format) {
        eprintln!("Error writing to stdout: {}", e);
        std::process::exit(1);
    }
}