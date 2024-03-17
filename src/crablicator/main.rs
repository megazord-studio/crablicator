use tokio;
use std::env;
use eventstore::{Client, ClientSettings, ReadAllOptions, StreamPosition, ResolvedEvent, AppendToStreamOptions, ExpectedRevision, EventData, RecordedEvent};
use std::error::Error;
use dotenv::dotenv;
use tokio::sync::mpsc;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod migrations;

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
        tx.send(event).await.expect("Failed to tx event");
    };
    Ok(())
}

async fn writer(
    client: &Client,
    original_event: &RecordedEvent,
) -> Result<(), Box<dyn Error>> {
    let event_type = original_event.event_type.clone();
    let event_data = original_event.data.clone();
    let options = AppendToStreamOptions::default()
        .expected_revision(ExpectedRevision::Any);
    let new_event: EventData = migrations::get_new_event(event_type, event_data).await?;
    client.append_to_stream(
        original_event.stream_id.to_string(),
        &options,
        new_event).await?;
    Ok(())
}

async fn worker(
    id: usize,
    client: &Client,
    mut rx: mpsc::Receiver<ResolvedEvent>
) {
    while let Some(event) = rx.recv().await {
        let original_event = event.get_original_event();
        if let Err(e) = writer(&client, original_event).await {
            eprintln!("Error processing event in worker {}: {}", id, e);
        }
    }
}

async fn write_events(
    client: &Client,
    rx: &mut mpsc::Receiver<ResolvedEvent>,
) -> Result<(), Box<dyn Error>> {
    let (txs, rxs): (Vec<_>, Vec<_>) = (
        0..WORKERS).map(|_| mpsc::channel::<ResolvedEvent>(100)).unzip();
    for (i, rx) in rxs.into_iter().enumerate() {
        let worker_client = client.clone();
        tokio::spawn(async move {
            worker(i, &worker_client, rx).await;
        });
    }
    while let Some(event) = rx.recv().await {
        let original_event = event.get_original_event();
        let event_stream_id = original_event.stream_id.clone();
        let mut hasher = DefaultHasher::new();
        event_stream_id.hash(&mut hasher);
        let worker_index = (hasher.finish() as usize) % WORKERS;
        if let Err(e) = txs[worker_index].send(event).await {
            eprintln!("Failed to send event to worker {}: {}", worker_index, e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let origin_settings = env::var("ORIGIN_CONNECTION_STRING")?
        .parse::<ClientSettings>()?;
    let destination_settings = env::var("DESTINATION_CONNECTION_STRING")?
        .parse::<ClientSettings>()?;
    let origin_client = Client::new(origin_settings)?;
    let destination_client = Client::new(destination_settings)?;
    let (tx, mut rx) = mpsc::channel::<ResolvedEvent>(10000);
    let reader_handle = tokio::spawn(async move {
        read_events(&origin_client, &tx)
            .await.expect("Failed to read event");
    });
    let writer_handle = tokio::spawn(async move {
        write_events(&destination_client, &mut rx)
            .await.expect("Failed to write event");
    });
    let _ = reader_handle.await;
    let _ = writer_handle.await;

    Ok(())
}
