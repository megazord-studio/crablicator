use tokio;
use std::env;
use eventstore::{Client, ClientSettings, ReadAllOptions, StreamPosition, ResolvedEvent, AppendToStreamOptions, ExpectedRevision, EventData};
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
        tx.send(event).await.expect("Failed to tx event");
    };
    Ok(())
}

async fn write_events(
    client: &Client,
    rx: &mut mpsc::Receiver<ResolvedEvent>,
) -> Result<(), Box<dyn Error>> {
    let options = AppendToStreamOptions::default()
        .expected_revision(ExpectedRevision::Any);
    while let Some(event) = rx.recv().await {
        let original_event = event.get_original_event();
        let new_event = EventData::binary(
            original_event.event_type.clone(),
            original_event.data.clone()
        );
        client.append_to_stream(
            event.get_original_event().stream_id.to_string(),
            &options,
            new_event).await?;
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
