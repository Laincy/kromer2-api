use crate::model::krist::Motd;
use crate::{KromerClient, KromerError};

#[must_use]
/// Creates an endpoint for getting the message of the day.
pub const fn get_motd() -> MotdEndpoint {
    MotdEndpoint {}
}

/// Message of the day endpoint.
///
/// See: <https://krist.dev/docs/#api-MiscellaneousGroup-GetMOTD>
pub struct MotdEndpoint;

impl MotdEndpoint {
    /// Fetches the message of the day
    /// # Errors
    /// See `KromerError` for information on other error variants.
    pub async fn get(&self, client: &KromerClient) -> Result<Motd, KromerError> {
        // We just directly query here since there should be no chance of the server throwing an
        // error unless something is fucked server side

        let url = client.url.join("/api/krist/motd")?;

        let res = client
            .http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Motd>()
            .await?;

        Ok(res)
    }
}
