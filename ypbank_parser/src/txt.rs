use crate::error::Error;
use crate::models::{Status, Transaction, TxType};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

/// read_from читает транзакции из источника, реализующего Read, в формате TXT.
/// ожидается, что источник соблюдает формат полей из Спецификации [README.md].
pub fn read_from<R: Read>(reader: R) -> Result<Vec<Transaction>, Error> {
    let reader = BufReader::new(reader); // убрано `mut`
    let mut lines = reader.lines();
    let mut transactions = Vec::new();
    let mut current_record = HashMap::new();

    while let Some(line) = lines.next() {
        let line = line?.trim().to_owned();
        if line.is_empty() {
            // Пустая строка разделяет записи
            if !current_record.is_empty() {
                transactions.push(parse_record(&current_record)?);
                current_record.clear();
            }
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            let value = if key == "DESCRIPTION" && value.starts_with('"') && value.ends_with('"') {
                &value[1..value.len()-1]
            } else {
                value
            };
            current_record.insert(key.to_owned(), value.to_owned());
        } else {
            return Err(Error::InvalidFormat(format!("invalid line: {}", line)));
        }
    }

    if !current_record.is_empty() {
        transactions.push(parse_record(&current_record)?);
    }
    Ok(transactions)
}

/// parse_record парсит запись из HashMap полей.
fn parse_record(map: &HashMap<String, String>) -> Result<Transaction, Error> {

    let tx_id = get_required(map, "TX_ID")?.parse()?;
    let tx_type = TxType::from_str(get_required(map, "TX_TYPE")?)?;
    let from_user_id = get_required(map, "FROM_USER_ID")?.parse()?;
    let to_user_id = get_required(map, "TO_USER_ID")?.parse()?;
    let amount = get_required(map, "AMOUNT")?.parse()?;
    let timestamp = get_required(map, "TIMESTAMP")?.parse()?;
    let status = Status::from_str(get_required(map, "STATUS")?)?;
    let description = map.get("DESCRIPTION").unwrap_or(&String::new()).to_owned();

    Ok(Transaction {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    })
}

fn get_required<'a>(map: &'a HashMap<String, String>, key: &str) -> Result<&'a String, Error> {
    map.get(key).ok_or_else(|| Error::InvalidFormat(format!("missing field {}", key)))
}

/// write_to записывает транзакции в текстовом формате в writer.
pub fn write_to<W: Write>(transactions: &[Transaction], mut writer: W) -> Result<(), Error> {
    for (i, tx) in transactions.iter().enumerate() {
        if i > 0 {
            writeln!(writer)?;
        }
        writeln!(writer, "TX_ID: {}", tx.tx_id)?;
        writeln!(writer, "TX_TYPE: {}", tx.tx_type.as_str())?;
        writeln!(writer, "FROM_USER_ID: {}", tx.from_user_id)?;
        writeln!(writer, "TO_USER_ID: {}", tx.to_user_id)?;
        writeln!(writer, "AMOUNT: {}", tx.amount)?;
        writeln!(writer, "TIMESTAMP: {}", tx.timestamp)?;
        writeln!(writer, "STATUS: {}", tx.status.as_str())?;
        writeln!(writer, "DESCRIPTION: \"{}\"", tx.description)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txt_roundtrip() {
        let txs = vec![
            Transaction {
                tx_id: 1234567890123456,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 9876543210987654,
                amount: 10000,
                timestamp: 1633036800000,
                status: Status::Success,
                description: "Terminal deposit".to_owned(),
            },
            Transaction {
                tx_id: 2312321321321321,
                tx_type: TxType::Transfer,
                from_user_id: 1231231231231231,
                to_user_id: 9876543210987654,
                amount: 1000,
                timestamp: 1633056800000,
                status: Status::Failure,
                description: "User transfer".to_owned(),
            },
        ];

        let mut buf = Vec::new();
        write_to(&txs, &mut buf).unwrap();

        let parsed = read_from(&buf[..]).unwrap();
        assert_eq!(txs, parsed);
    }
}