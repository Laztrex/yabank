use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use ypbank_parser::{Format, read_from};

/// Cli - интерфейс сравнения двух файлов транзакций.
/// file1: первый файл
/// file2: второй файл
/// format1: формат первого файла
/// format2: формат второго файла
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    file1: String,

    #[arg(long)]
    format1: String,

    #[arg(long)]
    file2: String,

    #[arg(long)]
    format2: String,
}

fn main() {
    let cli = Cli::parse();

    let format1 = match cli.format1.parse::<Format>() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Invalid format1 '{}': {}", cli.format1, e);
            std::process::exit(1);
        }
    };
    let format2 = match cli.format2.parse::<Format>() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Invalid format2 '{}': {}", cli.format2, e);
            std::process::exit(1);
        }
    };

    let file1 = match File::open(&cli.file1) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file1 '{}': {}", cli.file1, e);
            std::process::exit(1);
        }
    };
    let reader1 = BufReader::new(file1);
    let txs1 = match read_from(reader1, format1) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading file1: {}", e);
            std::process::exit(1);
        }
    };

    let file2 = match File::open(&cli.file2) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file2 '{}': {}", cli.file2, e);
            std::process::exit(1);
        }
    };
    let reader2 = BufReader::new(file2);
    let txs2 = match read_from(reader2, format2) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading file2: {}", e);
            std::process::exit(1);
        }
    };

    if txs1 == txs2 {
        println!("The transaction records are identical.");
    } else {
        println!("The transaction records differ.");

        for (i, (a, b)) in txs1.iter().zip(txs2.iter()).enumerate() {
            if a != b {
                println!("First difference at index {}:", i);
                println!("  File1: {:?}", a);
                println!("  File2: {:?}", b);
                break;
            }
        }
        if txs1.len() != txs2.len() {
            println!("Length mismatch: file1 has {}, file2 has {}", txs1.len(), txs2.len());
        }
    }
}