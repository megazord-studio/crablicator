use eventstore::EventData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TestEvent {
    pub id: String,
    pub important_data: String,
}

pub async fn transform_event(
    event_type: String,
    event_data: bytes::Bytes,
) -> Result<EventData, Box<dyn std::error::Error>> {
    if event_type == "TestEvent" {
        let mut event = serde_json::from_slice::<TestEvent>(&event_data)?;
        // transform event
        event.important_data = "new data".to_string();
        //
        return Ok(EventData::json(event_type.clone(), event)?);
    }
    return Ok(EventData::binary(event_type.clone(), event_data));
}
