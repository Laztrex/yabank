use crate::error::Error;
use crate::models::{Status, Transaction, TxType};
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// CsvRecord - структура для сериализации/десериализации CSV через либу serde.
/// поля соответствуют формату из Спецификации [README.md].
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct CsvRecord {
    tx_id: u64,
    tx_type: String,
    from_user_id: u64,
    to_user_id: u64,
    amount: i64,
    timestamp: u64,
    status: String,
    description: String,
}

impl From<&Transaction> for CsvRecord {
    fn from(tx: &Transaction) -> Self {
        CsvRecord {
            tx_id: tx.tx_id,
            tx_type: tx.tx_type.as_str().to_owned(),
            from_user_id: tx.from_user_id,
            to_user_id: tx.to_user_id,
            amount: tx.amount,
            timestamp: tx.timestamp,
            status: tx.status.as_str().to_owned(),
            description: tx.description.clone(),
        }
    }
}

impl TryFrom<CsvRecord> for Transaction {
    type Error = Error;

    fn try_from(rec: CsvRecord) -> Result<Self, Self::Error> {
        Ok(Transaction {
            tx_id: rec.tx_id,
            tx_type: TxType::from_str(&rec.tx_type)?,
            from_user_id: rec.from_user_id,
            to_user_id: rec.to_user_id,
            amount: rec.amount,
            timestamp: rec.timestamp,
            status: Status::from_str(&rec.status)?,
            description: rec.description,
        })
    }
}

/// read_from читает транзакции из источника, реализующего Read, в формате CSV.
/// ожидается, что источник соблюдает формат полей из Спецификации [README.md].
pub fn read_from<R: Read>(reader: R) -> Result<Vec<Transaction>, Error> {
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    let mut transactions = Vec::new();
    for result in csv_reader.deserialize() {
        let record: CsvRecord = result.map_err(|e| Error::InvalidFormat(e.to_string()))?;
        transactions.push(record.try_into()?);
    }
    Ok(transactions)
}

/// write_to записывает транзакции в структуру, реализующую Write, в формате CSV.
pub fn write_to<W: Write>(transactions: &[Transaction], writer: W) -> Result<(), Error> {
    let mut csv_writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(writer);

    for tx in transactions {
        let record = CsvRecord::from(tx);
        csv_writer.serialize(record).map_err(|e| Error::InvalidFormat(e.to_string()))?;
    }
    csv_writer.flush().map_err(Error::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Status, TxType};

    #[test]
    fn test_csv_roundtrip() {
        let txs = vec![
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
        ];

        let mut buf = Vec::new();
        write_to(&txs, &mut buf).unwrap();

        let parsed = read_from(&buf[..]).unwrap();
        assert_eq!(txs, parsed);
    }
}