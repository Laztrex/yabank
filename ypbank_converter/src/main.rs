use clap::Parser;
use std::fs::File;
use std::io::{stdout, BufReader, BufWriter};
use ypbank_parser::{Format, read_from, write_to};

/// Cli - интерфейс конвертера файлов транзакций между форматами.
/// input: входной файл
/// input_format: формат входного файла (csv, txt, bin)
/// output_format: формат выходного файла (csv, txt, bin)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    input: String,

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
            std::process::exit(1);
        }
    };
    let output_format = match cli.output_format.parse::<Format>() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Invalid output format '{}': {}", cli.output_format, e);
            std::process::exit(1);
        }
    };

    let file = match File::open(&cli.input) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open input file '{}': {}", cli.input, e);
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);

    let transactions = match read_from(reader, input_format) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading from '{}': {}", cli.input, e);
            std::process::exit(1);
        }
    };

    let writer = BufWriter::new(stdout().lock());
    if let Err(e) = write_to(&transactions, writer, output_format) {
        eprintln!("Error writing to stdout: {}", e);
        std::process::exit(1);
    }
}