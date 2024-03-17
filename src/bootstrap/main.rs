use tokio;
use std::env;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use eventstore::{ClientSettings, Client, EventData, AppendToStreamOptions, ExpectedRevision};
use std::error::Error;
use dotenv::dotenv;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize)]
struct TestEvent {
    pub id: String,
    pub important_data: String,
}

async fn send_event(
    client: &Client, stream: &str
) -> Result<(), Box<dyn Error>> {
    let data = TestEvent {
        id: Uuid::new_v4().to_string(),
        important_data: "some value".to_string(),
    };
    let evt = EventData::json("language-poll", data)?.id(Uuid::new_v4());
    let options = AppendToStreamOptions::default()
        .expected_revision(ExpectedRevision::NoStream);
    client
        .append_to_stream(stream, &options, evt)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let settings = env::var("CONNECTION_STRING")?
        .parse::<ClientSettings>()?;
    let client = Client::new(settings)?;

    let categories = vec!["apple", "banana", "cherry", "date", "elderberry"];
    for _ in 0..20 {
        let topic = categories.choose(&mut rand::thread_rng()).expect("Topics list is empty");
        println!("Sending event to topic {}", topic);
    }

    Ok(())
}
