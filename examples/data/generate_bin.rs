use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;
use ypbank_parser::{write_to, Format, Transaction, TxType, Status};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let data_dir = Path::new(&manifest_dir)
        .parent()
        .expect("should have parent")
        .join("examples/data");
    fs::create_dir_all(&data_dir)?;
    let bin_path = data_dir.join("example.bin");

    let transactions = vec![
        Transaction {
            tx_id: 1001,
            tx_type: TxType::Deposit,
            from_user_id: 0,
            to_user_id: 501,
            amount: 50000,
            timestamp: 1672531200000,
            status: Status::Success,
            description: "Initial account funding".to_owned(),
        },
        Transaction {
            tx_id: 1002,
            tx_type: TxType::Transfer,
            from_user_id: 501,
            to_user_id: 502,
            amount: 15000,
            timestamp: 1672534800000,
            status: Status::Failure,
            description: "Payment for services, invoice #123".to_owned(),
        },
        Transaction {
            tx_id: 1003,
            tx_type: TxType::Withdrawal,
            from_user_id: 502,
            to_user_id: 0,
            amount: 1000,
            timestamp: 1672538400000,
            status: Status::Pending,
            description: "ATM withdrawal".to_owned(),
        },
    ];

    let file = File::create(&bin_path)?;
    let writer = BufWriter::new(file);
    write_to(&transactions, writer, Format::Bin)?;
    println!("Generated {}", bin_path.display());
    Ok(())
}