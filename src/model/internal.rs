//! Internal types used to create requests and deserialize data

use crate::{
    KromerError,
    model::krist::{Address, Transaction},
};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

pub trait ExtractJson<T>
where
    T: Sized,
{
    fn extract(self) -> Result<T, KromerError>;
}

#[derive(Debug, Clone, Copy)]
/// Provides tha ability to paginate a variety
pub struct Paginator {
    // Using a u16 here because there is little chance that any kromer server has more than 2^16
    // players. Will accept a change if the need arises
    pub(crate) limit: u16,
    pub(crate) offset: u16,
}

impl Default for Paginator {
    fn default() -> Self {
        Self {
            limit: 50,
            offset: 0,
        }
    }
}

impl Paginator {
    #[allow(dead_code)]
    /// Use when inserting homogeneous query params into a URI
    pub(crate) fn add_map(self, map: &mut HashMap<&'static str, String>) {
        map.insert("limit", format!("{:?}", self.limit));

        map.insert("offset", format!("{:?}", self.offset));
    }

    /// Use when inserting only the paginator params
    pub(crate) const fn direct_query(self) -> [(&'static str, u16); 2] {
        [("limit", self.limit), ("offset", self.offset)]
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum GetAddressRes {
    Address { address: Address },
    KristError { error: String, message: String },
}

impl ExtractJson<Address> for GetAddressRes {
    fn extract(self) -> Result<Address, KromerError> {
        match self {
            Self::Address { address } => Ok(address),
            Self::KristError { error, message } => Err(KromerError::Krist { error, message }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ListAddressesRes {
    Addresses { addresses: Vec<Address> },
    KristError { error: String, message: String },
}

impl ExtractJson<Vec<Address>> for ListAddressesRes {
    fn extract(self) -> Result<Vec<Address>, KromerError> {
        match self {
            Self::Addresses { addresses } => Ok(addresses),
            Self::KristError { error, message } => Err(KromerError::Krist { error, message }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AddressTransactionsRes {
    Transaction { transactions: Vec<Transaction> },
    KristError { error: String, message: String },
}

impl ExtractJson<Vec<Transaction>> for AddressTransactionsRes {
    fn extract(self) -> Result<Vec<Transaction>, KromerError> {
        match self {
            Self::Transaction { transactions } => Ok(transactions),
            Self::KristError { error, message } => Err(KromerError::Krist { error, message }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser() {
        let res = serde_json::from_str::<AddressTransactionsRes>(
            r#"{"ok":true,"count":5,"total":888,"transactions":[
            {"id":1413,"from":"serverwelf","to":"ks0d5iqb6p","value":1.5,"time":"2025-08-02T02:59:59.823081+00:00","metadata":"","type":"mined"},
            {"id":1412,"from":"serverwelf","to":"ks0d5iqb6p","value":1.5,"time":"2025-08-02T01:59:59.170857+00:00","metadata":"","type":"unknown"},
            {"id":1410,"from":"serverwelf","to":"kfl50b9gsq","value":2.25,"time":"2025-08-02T00:59:59.195028+00:00","metadata":"","type":"name_purchase"},
            {"id":1409,"from":"serverwelf","to":"ks0d5iqb6p","value":2.25,"time":"2025-08-02T00:59:59.191216+00:00","metadata":"","type":"name_a_record"},
            {"id":1408,"from":"serverwelf","to":"k9sl41ktqj","value":2.25,"time":"2025-08-02T00:59:59.186701+00:00","metadata":"","type":"transfer"}]}"#
        ).unwrap();
    }
}
