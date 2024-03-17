use tokio;
// use uuid::Uuid;
use std::env;
use serde::{Serialize, Deserialize};
//use eventstore::{Client, ClientSettings, EventData};
// use eventstore::{ClientSettings};
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
    println!("{}", env::var("CONNECTION_STRING")?);
    //let connection_string = env::var("CONNECTION_STRING")?
        //.parse::<ClientSettings>()?;
    // println!("{:?}", connection_string);

    Ok(())
}
