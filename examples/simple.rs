use kromer2_api::KromerClient;
use kromer2_api::endpoints::get_motd;

#[tokio::main]
async fn main() -> Result<(), kromer2_api::KromerError> {
    tracing_subscriber::fmt::init();

    let client = KromerClient::new("https://kromer.reconnected.cc").unwrap();

    let req = get_motd().get(&client).await?;

    print!("{req:?}");

    Ok(())
}
