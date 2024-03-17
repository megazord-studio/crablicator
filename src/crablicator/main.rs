use tokio;
use std::error::Error;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    println!("{}", env::var("CONNECTION_STRING")?);
    Ok(())
}
