//! Библиотека для парсинга и сериализации финансовых данных в форматах YPBank.
//! Поддерживаются форматы:
//! - CSV (YPBankCsv)
//! - Текстовый (YPBankText) - ключ-значение
//! - Бинарный (YPBankBin)
//!
//! Все операции чтения/записи используют трейты `Read` и `Write` из стандартной библиотеки,
//! что позволяет работать с файлами, буферами, сетевыми потоками и т.д.

pub mod error;
pub mod models;
pub mod csv;
pub mod txt;
pub mod bin;

pub use error::Error;
pub use models::{Status, Transaction, TxType};

/// read_from читает транзакции из источника в указанном формате.
pub fn read_from<R: std::io::Read>(reader: R, format: Format) -> Result<Vec<Transaction>, Error> {
    match format {
        Format::Csv => csv::read_from(reader),
        Format::Txt => txt::read_from(reader),
        Format::Bin => bin::read_from(reader),
    }
}

/// write_to записывает транзакции в структуру в указанном формате.
pub fn write_to<W: std::io::Write>(transactions: &[Transaction], writer: W, format: Format) -> Result<(), Error> {
    match format {
        Format::Csv => csv::write_to(transactions, writer),
        Format::Txt => txt::write_to(transactions, writer),
        Format::Bin => bin::write_to(transactions, writer),
    }
}

/// Поддерживаемые форматы.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Csv,
    Txt,
    Bin,
}

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(Format::Csv),
            "txt" | "text" => Ok(Format::Txt),
            "bin" | "binary" => Ok(Format::Bin),
            _ => Err(format!("unknown format: {}", s)),
        }
    }
}