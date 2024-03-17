use tokio;
use std::env;
use eventstore::{Client, ClientSettings, ReadAllOptions, StreamPosition};
use std::error::Error;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let settings = env::var("CONNECTION_STRING")?
        .parse::<ClientSettings>()?;
      let client = Client::new(settings)?;
    let options = ReadAllOptions::default()
        .position(StreamPosition::Start)
        .forwards();
    let mut stream = client.read_all(&options).await?;
    while let Some(event) = stream.next().await? {
        let test_event = event.get_original_event();
        println!("Event> {:?}", test_event);
    }
    Ok(())
}
