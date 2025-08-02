//! Internal types used to create requests and deserialize data

use crate::KromerError;
use crate::model::krist::Address;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

pub trait ExtractJson<T>
where
    T: Sized,
{
    fn extract(self) -> Result<T, KromerError>;
}

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
    pub(crate) fn add_map(&self, map: &mut HashMap<&'static str, String>) {
        map.insert("limit", format!("{:?}", self.limit));

        map.insert("offset", format!("{:?}", self.offset));
    }

    /// Use when inserting only the paginator params
    pub(crate) const fn direct_query(&self) -> [(&'static str, u16); 2] {
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
