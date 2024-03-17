use tokio;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use eventstore::{Client, ClientSettings, EventData};
use std::error::Error;
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
struct TestEvent {
    id: String,
    important_data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let connection_string = env::var("CONNECTION_STRING")?
        .parse::<ClientSettings>()?;
    let settings = ClientSettings::parse(connection_string)?;

    let client = Client::new(settings)?;
    Ok(())
}
