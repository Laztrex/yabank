use std::io;
use std::num::{ParseIntError, TryFromIntError};
use std::string::FromUtf8Error;

/// Общий тип ошибки для всех операций парсинга/сериализации.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidFormat(String),
    InvalidValue(String),
    ParseInt(ParseIntError),
    TryFromInt(TryFromIntError),
    Utf8(FromUtf8Error),
    UnknownTxType(u8),
    UnknownStatus(u8),
    InvalidMagic,
    Other(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::TryFromInt(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::Utf8(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            Error::InvalidValue(s) => write!(f, "Invalid value: {}", s),
            Error::ParseInt(e) => write!(f, "Parse int error: {}", e),
            Error::TryFromInt(e) => write!(f, "Try from int error: {}", e),
            Error::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            Error::UnknownTxType(b) => write!(f, "Unknown transaction type byte: {}", b),
            Error::UnknownStatus(b) => write!(f, "Unknown status byte: {}", b),
            Error::InvalidMagic => write!(f, "Invalid magic number in binary record"),
            Error::Other(s) => write!(f, "Other error: {}", s),
        }
    }
}

impl std::error::Error for Error {}