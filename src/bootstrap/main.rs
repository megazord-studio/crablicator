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

    let event = TestEvent {
        id: Uuid::new_v4().to_string(),
        important_data: "I wrote my first event!".to_string(),
    };

    // Serialize the event into JSON.
    let event_data = EventData::json("TestEvent", &event)?
        .id(Uuid::new_v4()); // Associate a unique identifier with the event data

    // Append the event data to a stream.
    client
        .append_to_stream("some-stream", &Default::default(), event_data)
        .await?;

    Ok(())
}
