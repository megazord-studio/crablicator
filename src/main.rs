//use eventstore::{Client, EventData};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    is_rust_a_nice_language: bool,
}

//const MAX_WORKERS: usize = 10;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let settings = "esdb://admin:changeit@localhost:2113".parse()?;
    //let client = Client::new(settings)?;

    // Example list of streams to read from. This should come from your input.
    let streams = vec!["stream1", "stream2", "stream3"];

    for stream_name in streams.into_iter() {
        println!("{stream_name}")
    }

    // Some mechanism to wait for all tasks to complete or a signal to shutdown.
    // This example lacks such mechanism for brevity.
    Ok(())
}
