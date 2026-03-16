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