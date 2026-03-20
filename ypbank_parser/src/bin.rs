use crate::error::Error;
use crate::models::{Status, Transaction, TxType};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

const MAGIC: u32 = 0x59_50_42_4E;

pub fn read_from<R: Read>(mut reader: R) -> Result<Vec<Transaction>, Error> {
    let mut transactions = Vec::new();
    loop {
        let mut header = [0u8; 8];
        if let Err(e) = reader.read_exact(&mut header) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break;
            }
            return Err(Error::Io(e));
        }

        let magic = (&header[0..4]).read_u32::<BigEndian>()?;
        if magic != MAGIC {
            return Err(Error::InvalidMagic);
        }
        let record_size = (&header[4..8]).read_u32::<BigEndian>()? as usize;

        let mut body = vec![0u8; record_size];
        reader.read_exact(&mut body)?;
        let mut cursor = std::io::Cursor::new(body);

        let tx_id = cursor.read_u64::<BigEndian>()?;
        let tx_type_byte = cursor.read_u8()?;
        let from_user_id = cursor.read_u64::<BigEndian>()?;
        let to_user_id = cursor.read_u64::<BigEndian>()?;
        let amount = cursor.read_i64::<BigEndian>()?;
        let timestamp = cursor.read_u64::<BigEndian>()?;
        let status_byte = cursor.read_u8()?;
        let desc_len = cursor.read_u32::<BigEndian>()? as usize;

        let description = if desc_len > 0 {
            let mut desc_bytes = vec![0u8; desc_len];
            cursor.read_exact(&mut desc_bytes)?;
            String::from_utf8(desc_bytes)?
        } else {
            String::new()
        };

        if cursor.position() as usize != record_size {
            return Err(Error::InvalidFormat("record size mismatch".to_owned()));
        }

        let tx_type = TxType::from_u8(tx_type_byte)?;
        let status = Status::from_u8(status_byte)?;

        transactions.push(Transaction {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        });
    }
    Ok(transactions)
}

pub fn write_to<W: Write>(transactions: &[Transaction], mut writer: W) -> Result<(), Error> {
    for tx in transactions {
        let mut body = Vec::new();

        body.write_u64::<BigEndian>(tx.tx_id)?;
        body.write_u8(tx.tx_type.to_u8())?;
        body.write_u64::<BigEndian>(tx.from_user_id)?;
        body.write_u64::<BigEndian>(tx.to_user_id)?;
        body.write_i64::<BigEndian>(tx.amount)?;
        body.write_u64::<BigEndian>(tx.timestamp)?;
        body.write_u8(tx.status.to_u8())?;

        let desc_bytes = tx.description.as_bytes();
        body.write_u32::<BigEndian>(desc_bytes.len() as u32)?;
        body.write_all(desc_bytes)?;

        let record_size = body.len() as u32;

        writer.write_u32::<BigEndian>(MAGIC)?;
        writer.write_u32::<BigEndian>(record_size)?;
        writer.write_all(&body)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_roundtrip() {
        let txs = vec![
            Transaction {
                tx_id: 1001,
                tx_type: TxType::Deposit,
                from_user_id: 0,
                to_user_id: 501,
                amount: 50000,
                timestamp: 1672531200000,
                status: Status::Success,
                description: "Initial funding".to_owned(),
            },
            Transaction {
                tx_id: 1002,
                tx_type: TxType::Transfer,
                from_user_id: 501,
                to_user_id: 502,
                amount: 15000,
                timestamp: 1672534800000,
                status: Status::Failure,
                description: "Payment".to_owned(),
            },
            Transaction {
                tx_id: 1003,
                tx_type: TxType::Withdrawal,
                from_user_id: 502,
                to_user_id: 0,
                amount: 1000,
                timestamp: 1672538400000,
                status: Status::Pending,
                description: "".to_owned(), // пустое описание
            },
        ];

        let mut buf = Vec::new();
        write_to(&txs, &mut buf).unwrap();

        let parsed = read_from(&buf[..]).unwrap();
        assert_eq!(txs, parsed);
    }

    #[test]
    fn test_bin_invalid_magic() {
        let mut buf = Vec::new();
        // Записываем неправильное магическое число
        buf.write_u32::<BigEndian>(0xDEADBEEF).unwrap();
        buf.write_u32::<BigEndian>(0).unwrap(); // record_size = 0

        let result = read_from(&buf[..]);
        assert!(matches!(result, Err(Error::InvalidMagic)));
    }

    #[test]
    fn test_bin_empty_input() {
        let buf = vec![];
        let result = read_from(&buf[..]);
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_bin_truncated_record() {
        let txs = vec![Transaction {
            tx_id: 1001,
            tx_type: TxType::Deposit,
            from_user_id: 0,
            to_user_id: 501,
            amount: 50000,
            timestamp: 1672531200000,
            status: Status::Success,
            description: "Test".to_owned(),
        }];

        let mut buf = Vec::new();
        write_to(&txs, &mut buf).unwrap();

        // Обрезаем буфер (убираем часть данных)
        buf.truncate(buf.len() - 5);
        let result = read_from(&buf[..]);
        assert!(matches!(result, Err(Error::Io(_)))); // должна быть ошибка чтения
    }
}
