//! Exported types modelling Kromer2's Krist API.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Information about an address fetched from the API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    #[allow(clippy::struct_field_names)]
    pub address: String,
    pub balance: Decimal,
    #[serde(alias = "totalin")]
    pub total_in: Decimal,
    #[serde(alias = "totalout")]
    pub total_out: Decimal,
    #[serde(alias = "firstseen")]
    pub first_seen: DateTime<Utc>,
    pub names: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// Message of the day
pub struct Motd {
    // pub server_time: DateTime<Utc>,
    #[serde(alias = "motd")]
    pub msg: String,
    pub public_url: String,
    pub public_ws_url: String,
    pub transactions_enabled: bool,
    pub debug_mode: bool,
    pub package: Package,
    pub currency: Currency,
    pub notice: String,
}

/// The package section of the [Motd] struct
///
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub author: String,
    // fucking euros
    #[serde(alias = "licence")]
    pub license: String,
    pub repository: String,
    // // TODO: uncomment this when the commit hash fix hits prod
    // pub git_hash: String,
}

/// The currency section of the [Motd] struct
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Currency {
    pub address_prefix: String,
    pub name_suffix: String,
    #[serde(alias = "currency_name")]
    pub name: String,
    #[serde(alias = "currency_symbol")]
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub id: u32,
    pub from: Option<String>,
    pub to: String,
    pub value: Decimal,
    pub time: DateTime<Utc>,
    pub name: Option<String>,
    // // TODO: Implement metadata parsing
    pub metadata: Option<String>,
    pub sent_metaname: Option<String>,
    pub sent_name: Option<String>,
    #[allow(clippy::struct_field_names)]
    #[serde(alias = "type")]
    pub transaction_type: TransactionType,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Mined,
    Unknown,
    NamePurchase,
    NameARecord,
    NameTransfer,
    Transfer,
}
