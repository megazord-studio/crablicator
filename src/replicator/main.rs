use dotenv::dotenv;
use eventstore::{
    AppendToStreamOptions, Client, EventData, ExpectedRevision, ReadAllOptions, ResolvedEvent,
    StreamPosition,
};
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::error::Error;
use std::hash::{Hash, Hasher};
use tokio;
use tokio::sync::mpsc;

use crablicator::util::esdb;
// use crablicator::util::stats;
mod dashboard;
mod transformations;

const WORKERS: usize = 10;
const CHANNEL_BUFFER_SIZE: usize = 1000000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let reader_client = esdb::get_client(&env::var("REPLICATOR_READER_CONNECTION_STRING")?)?;
    let writer_client = esdb::get_client(&env::var("REPLICATOR_WRITER_CONNECTION_STRING")?)?;
    let (tx, mut rx) = mpsc::channel::<ResolvedEvent>(CHANNEL_BUFFER_SIZE);
    let reader_handle = tokio::spawn(async move {
        read_events(&reader_client, &tx)
            .await
            .expect("Error in read_events");
    });
    let writer_handle = tokio::spawn(async move {
        write_events(&writer_client, &mut rx)
            .await
            .expect("Error in write_events");
    });

    let server_handle = dashboard::run_server();
    futures::future::join_all([reader_handle, writer_handle, server_handle]).await;
    Ok(())
}

async fn read_events(
    client: &Client,
    tx: &mpsc::Sender<ResolvedEvent>,
) -> Result<(), Box<dyn Error>> {
    let options = ReadAllOptions::default()
        .position(StreamPosition::Start)
        .forwards();
    let mut stream = client.read_all(&options).await?;
    while let Some(event) = stream.next().await? {
        tx.send(event)
            .await
            .expect("Failed to send event from read_events to write_events");
    }
    Ok(())
}

async fn write_events(
    client: &Client,
    rx: &mut mpsc::Receiver<ResolvedEvent>,
) -> Result<(), Box<dyn Error>> {
    let (txs, rxs): (Vec<_>, Vec<_>) = (0..WORKERS)
        .map(|_| mpsc::channel::<ResolvedEvent>(CHANNEL_BUFFER_SIZE))
        .unzip();
    for rx in rxs.into_iter() {
        let worker_client = client.clone();
        tokio::spawn(async move {
            worker(&worker_client, rx).await;
        });
    }
    while let Some(event) = rx.recv().await {
        let event_stream_id = event.get_original_event().stream_id.clone();
        let mut hasher = DefaultHasher::new();
        event_stream_id.hash(&mut hasher);
        let worker_index = (hasher.finish() as usize) % WORKERS;
        if let Err(e) = txs[worker_index].send(event).await {
            eprintln!("Failed to send event to worker {}: {}", worker_index, e);
        }
    }

    Ok(())
}

async fn worker(client: &Client, mut rx: mpsc::Receiver<ResolvedEvent>) {
    while let Some(event) = rx.recv().await {
        if let Err(e) = append_to_stream(&client, &event).await {
            eprintln!("Error in append_to_stream {}", e);
        }
    }
}

async fn append_to_stream(client: &Client, event: &ResolvedEvent) -> Result<(), Box<dyn Error>> {
    let original_event = event.get_original_event();
    let new_event: EventData = transformations::transform_event(
        original_event.event_type.clone(),
        original_event.data.clone(),
    )
    .await?;
    let options = AppendToStreamOptions::default().expected_revision(ExpectedRevision::Any);
    client
        .append_to_stream(original_event.stream_id.to_string(), &options, new_event)
        .await?;
    Ok(())
}
