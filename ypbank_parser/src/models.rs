use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TxType {
    Deposit,
    Transfer,
    Withdrawal,
}

impl TxType {
    pub fn to_u8(&self) -> u8 {
        match self {
            TxType::Deposit => 0,
            TxType::Transfer => 1,
            TxType::Withdrawal => 2,
        }
    }

    pub fn from_u8(byte: u8) -> Result<Self, super::Error> {
        match byte {
            0 => Ok(TxType::Deposit),
            1 => Ok(TxType::Transfer),
            2 => Ok(TxType::Withdrawal),
            _ => Err(super::Error::UnknownTxType(byte)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TxType::Deposit => "DEPOSIT",
            TxType::Transfer => "TRANSFER",
            TxType::Withdrawal => "WITHDRAWAL",
        }
    }
}

impl FromStr for TxType {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TxType::Deposit),
            "TRANSFER" => Ok(TxType::Transfer),
            "WITHDRAWAL" => Ok(TxType::Withdrawal),
            _ => Err(super::Error::InvalidValue(format!("unknown TX_TYPE: {}", s))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Success,
    Failure,
    Pending,
}

impl Status {
    pub fn to_u8(&self) -> u8 {
        match self {
            Status::Success => 0,
            Status::Failure => 1,
            Status::Pending => 2,
        }
    }

    pub fn from_u8(byte: u8) -> Result<Self, super::Error> {
        match byte {
            0 => Ok(Status::Success),
            1 => Ok(Status::Failure),
            2 => Ok(Status::Pending),
            _ => Err(super::Error::UnknownStatus(byte)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Success => "SUCCESS",
            Status::Failure => "FAILURE",
            Status::Pending => "PENDING",
        }
    }
}

impl FromStr for Status {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(Status::Success),
            "FAILURE" => Ok(Status::Failure),
            "PENDING" => Ok(Status::Pending),
            _ => Err(super::Error::InvalidValue(format!("unknown STATUS: {}", s))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TxType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: Status,
    pub description: String,
}
