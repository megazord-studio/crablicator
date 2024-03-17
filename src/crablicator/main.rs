use tokio;
use std::env;
//use eventstore::{Client, ClientSettings, ReadAllOptions, StreamPosition, EventData, AppendToStreamOptions, ExpectedRevision, ReadResult};
use eventstore::{Client, ClientSettings, ReadAllOptions, StreamPosition, EventData, ReadEventResult, ResolvedEvent};
use std::error::Error;
use dotenv::dotenv;
use tokio::sync::mpsc;

const WORKERS: usize = 10;

async fn read_events(
    client: &Client,
    tx: &mpsc::Sender<ResolvedEvent>,
) -> Result<(), Box<dyn Error>> {
    let options = ReadAllOptions::default()
        .position(StreamPosition::Start)
        .forwards();
    let mut stream = client.read_all(&options).await?;
    while let Some(event) = stream.next().await? {
        tx.send(event).await.expect("Failed to send tx event");
    };
    Ok(())
}

async fn write_events(
    _client: &Client,
    rx: &mut mpsc::Receiver<ResolvedEvent>,
) -> Result<(), Box<dyn Error>> {
    while let Some(event) = rx.recv().await {
//        let stream = event
//        let options = AppendToStreamOptions::default()
//            .expected_revision(ExpectedRevision::Any);
//        client.append_to_stream(stream.clone(), &options, evt).await?;
        println!("Event> {:?}", event);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let settings = env::var("CONNECTION_STRING")?
        .parse::<ClientSettings>()?;
    let client = Client::new(settings)?;
    let client_clone = client.clone();
    let (tx, mut rx) = mpsc::channel::<ResolvedEvent>(WORKERS);

    let reader_handle = tokio::spawn(async move {
        read_events(&client_clone, &tx).await.expect("Failed to read event");
    });

    // Spawn write_events task and get its JoinHandle
    let writer_handle = tokio::spawn(async move {
        write_events(&client, &mut rx).await.expect("Failed to write event");
    });
    let _ = reader_handle.await;
    let _ = writer_handle.await;

    Ok(())
}
