use dotenv::dotenv;
use eventstore::{AppendToStreamOptions, Client, ClientSettings, EventData, ExpectedRevision};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use tokio;
use uuid::Uuid;

mod categories;

#[derive(Serialize, Deserialize)]
struct TestEvent {
    pub id: String,
    pub important_data: String,
}

async fn send_event(client: &Client, category: &str) -> Result<(), Box<dyn Error>> {
    loop {
        let stream_name = format!("{}-{}", category, Uuid::new_v4());
        let event_count = rand::thread_rng().gen_range(10..5000);
        for _ in 0..event_count {
            let data = TestEvent {
                id: Uuid::new_v4().to_string(),
                important_data: "Hello World".to_string(),
            };
            let event = EventData::json("TestEvent", data)?.id(Uuid::new_v4());
            let options = AppendToStreamOptions::default().expected_revision(ExpectedRevision::Any);
            client
                .append_to_stream(stream_name.clone(), &options, event)
                .await?;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let settings = env::var("CONNECTION_STRING")?.parse::<ClientSettings>()?;
    let client = Client::new(settings)?;
    let futures = categories::FRUITS.iter().map(|category| {
        let client_clone = client.clone();
        let category = category.to_string();
        tokio::spawn(async move {
            send_event(&client_clone, &category)
                .await
                .expect("Failed to send event");
        })
    });
    futures::future::join_all(futures).await;
    Ok(())
}
