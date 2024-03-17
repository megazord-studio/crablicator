use tokio;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
struct TestEvent {
    id: String,
    important_data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    println!("{}", env::var("CONNECTION_STRING")?);
    Ok(())
}
