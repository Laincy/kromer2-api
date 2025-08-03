use hashbrown::HashMap;

use crate::{
    KromerClient, KromerError,
    model::{
        internal::{AddressTransactionsRes, GetAddressRes, ListAddressesRes, Paginator},
        krist::{Address, Transaction},
    },
};

const ENDPOINT_URL: &str = "/api/krist/addresses";

/// Create an endpoint to fetch information about an address
pub fn get_address(addr: impl Into<String>) -> GetAddressEndpoint {
    GetAddressEndpoint {
        addr: addr.into(),
        names: None,
    }
}

pub struct GetAddressEndpoint {
    pub(crate) addr: String,
    pub(crate) names: Option<bool>,
}

impl GetAddressEndpoint {
    #[must_use]
    /// Sets the fetchNames query parameter.
    pub const fn fetch_names(mut self, fetch: bool) -> Self {
        self.names = Some(fetch);
        self
    }

    /// Fetches information about the provided address
    /// # Errors
    /// Function will return `KromerError::Krist` if the address does not exist or there is a
    /// server side error. See `KromerError` for information on other error variants.
    pub async fn get(&self, client: &KromerClient) -> Result<Address, KromerError> {
        let query = self.names.map(|val| [("fetchNames", val)]);

        client
            .get::<GetAddressRes, _, _>(&format!("{ENDPOINT_URL}/{}", self.addr), query)
            .await
    }
}

#[must_use]
/// Create an endpoint to get a paginated list of addresses.
pub fn list_addresses() -> ListAddressEndpoint {
    ListAddressEndpoint::default()
}

#[derive(Default)]
pub struct ListAddressEndpoint(Paginator);

impl ListAddressEndpoint {
    #[must_use]
    /// Sets the maximum number of reponses you want, capped at 1000.
    pub const fn limit(mut self, limit: u16) -> Self {
        if limit < 1000 {
            self.0.limit = limit;
        }
        self
    }

    #[must_use]
    /// Sets number of addresses you'd like your query to be offset by
    pub const fn offset(mut self, offset: u16) -> Self {
        self.0.offset = offset;
        self
    }

    /// Fetches information about the provided address
    ///
    /// # Errors
    /// See `KromerError` for information on other error variants.
    pub async fn get(&self, client: &KromerClient) -> Result<Vec<Address>, KromerError> {
        client
            .get::<ListAddressesRes, _, _>(ENDPOINT_URL, Some(self.0.direct_query()))
            .await
    }
}

#[must_use]
/// Create an endpoint to get a paginated list of addresses.
pub fn list_richest() -> ListRichestEndpoint {
    ListRichestEndpoint::default()
}

#[derive(Default)]
pub struct ListRichestEndpoint(Paginator);

impl ListRichestEndpoint {
    /// Sets the maximum number of reponses you want, capped at 1000.
    #[must_use]
    pub const fn limit(mut self, limit: u16) -> Self {
        if limit < 1000 {
            self.0.limit = limit;
        }
        self
    }

    #[must_use]
    /// Sets number of addresses you'd like your query to be offset by
    pub const fn offset(mut self, offset: u16) -> Self {
        self.0.offset = offset;
        self
    }

    /// Fetches information about the provided address
    /// # Errors
    /// See `KromerError` for information on other error variants.
    pub async fn get(&self, client: &KromerClient) -> Result<Vec<Address>, KromerError> {
        client
            .get::<ListAddressesRes, _, _>("/api/krist/addresses/rich", Some(self.0.direct_query()))
            .await
    }
}

#[must_use]
pub fn list_address_transactions(addr: impl Into<String>) -> AddressTransactionsEndpoint {
    AddressTransactionsEndpoint {
        addr: addr.into(),
        mined: false,
        ..Default::default()
    }
}

#[derive(Debug, Default, Clone)]
pub struct AddressTransactionsEndpoint {
    pub(crate) addr: String,
    pub(crate) page: Paginator,
    /// excludeMined query param
    pub(crate) mined: bool,
}

impl AddressTransactionsEndpoint {
    #[must_use]
    pub const fn limit(mut self, limit: u16) -> Self {
        if limit < 1000 {
            self.page.limit = limit;
        }
        self
    }

    #[must_use]
    /// Sets number of addresses you'd like your query to be offset by
    pub const fn offset(mut self, offset: u16) -> Self {
        self.page.offset = offset;
        self
    }

    #[must_use]
    /// Sets the excludeMined query param
    pub const fn exclude_mined(mut self, b: bool) -> Self {
        self.mined = b;
        self
    }

    /// Fetches information about the provided address' recent transactions as a paginated list.
    /// # Errors
    /// Function will return `KromerError::Krist` if the address does not exist or there is a
    /// server side error. See `KromerError` for information on other error variants.
    pub async fn get(&self, client: &KromerClient) -> Result<Vec<Transaction>, KromerError> {
        let mut params = HashMap::with_capacity(3);
        self.page.add_map(&mut params);
        params.insert("excludeMined", format!("{}", self.mined));

        client
            .get::<AddressTransactionsRes, _, _>(
                &format!("/api/krist/addresses/{}/transactions", self.addr),
                Some(params),
            )
            .await
    }
}
