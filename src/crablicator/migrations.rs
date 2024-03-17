use serde::{Serialize, Deserialize};
use eventstore::EventData;

#[derive(Serialize, Deserialize)]
struct TestEvent {
    pub id: String,
    pub important_data: String,
}

pub async fn get_new_event(
    event_type: String,
    event_data: bytes::Bytes,
) -> Result<EventData, Box<dyn std::error::Error>> {
    let new_event: EventData;
    if event_type == "TestEvent" {
        let mut test_event = serde_json::from_slice::<TestEvent>(&event_data)?;
        test_event.important_data = "new data".to_string();
        new_event = EventData::json(event_type.clone(), test_event)?;
    } else {
        new_event = EventData::binary(event_type.clone(), event_data);
    }
    Ok(new_event)
}
