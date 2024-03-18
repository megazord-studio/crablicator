use eventstore::{Client, ClientSettings};
use std::error::Error;

pub fn get_client(connection_string: &String) -> Result<Client, Box<dyn Error>> {
    let settings = connection_string.to_string().parse::<ClientSettings>()?;
    Ok(Client::new(settings)?)
}
