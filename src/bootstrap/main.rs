use tokio;
use std::env;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use eventstore::{Client, ClientSettings, EventData, AppendToStreamOptions, ExpectedRevision};
use std::error::Error;
use dotenv::dotenv;
use rand::Rng;
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize)]
struct TestEvent {
    pub id: String,
    pub important_data: String,
}

async fn send_event(
    client: &Client, category: &str
) -> Result<(), Box<dyn Error>> {
    loop {
        let stream = format!("{}-{}", category, Uuid::new_v4());
        let events = rand::thread_rng().gen_range(10..3000);
        for _ in 0..events {
            let data = TestEvent {
                id: Uuid::new_v4().to_string(),
                important_data: "Hello World".to_string(),
            };
            let evt = EventData::json("TestEvent", data)?
                .id(Uuid::new_v4());
            let options = AppendToStreamOptions::default()
                .expected_revision(ExpectedRevision::Any);
            client.append_to_stream(stream.clone(), &options, evt).await?;
        }
        sleep(Duration::from_millis(1000)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let settings = env::var("CONNECTION_STRING")?
        .parse::<ClientSettings>()?;
      let client = Client::new(settings)?;

    let categories = vec![
        "apple", "banana", "cherry", "date", "elderberry",
        "fig", "grape", "honeydew", "kiwi", "lemon",
        "mango", "nectarine", "orange", "pear", "quince",
        "raspberry", "strawberry", "tangerine", "watermelon",
        "blueberry", "blackberry", "cranberry", "gooseberry",
        "guava", "papaya", "pineapple", "plum", "pomegranate",
        "apricot", "avocado", "coconut", "grapefruit", "lime",
        "lychee", "olive", "peach", "persimmon", "pumpkin",
        "tomato", "zucchini", "artichoke", "asparagus", "beet",
        "broccoli", "brussels sprout", "cabbage", "carrot", "cauliflower",
        "celery", "corn", "cucumber", "eggplant", "green bean",
        "green pepper", "kale", "leek", "lettuce", "mushroom",
        "onion", "pea", "potato", "pumpkin", "radish",
        "red pepper", "spinach", "squash", "sweet potato", "swiss chard",
        "turnip", "yam", "zucchini", "artichoke", "asparagus",
        "beet", "broccoli", "brussels sprout", "cabbage", "carrot",
        "cauliflower", "celery", "corn", "cucumber", "eggplant",
        "green bean", "green pepper", "kale", "leek", "lettuce",
        "mushroom", "onion", "pea", "potato", "pumpkin",
        "radish", "red pepper", "spinach", "squash", "sweet potato",
        "swiss chard", "turnip", "yam", "zucchini", "artichoke",
        "asparagus", "beet", "broccoli", "brussels sprout", "cabbage",
        "carrot", "cauliflower", "celery", "corn", "cucumber",
        "eggplant", "green bean", "green pepper", "kale", "leek",
        "lettuce", "mushroom", "onion", "pea", "potato",
        "pumpkin", "radish", "red pepper", "spinach", "squash",
        "sweet potato", "swiss chard", "turnip", "yam", "zucchini",
        "artichoke", "asparagus", "beet", "broccoli", "brussels sprout",
        "cabbage", "carrot", "cauliflower", "celery", "corn",
        "cucumber", "eggplant", "green bean", "green pepper", "kale",
        "leek", "lettuce", "mushroom", "onion", "pea",
        "potato", "pumpkin", "radish", "red pepper", "spinach",
    ];

    let futures = categories.iter().map(|category| {
        let client_clone = client.clone();
        let category = category.to_string();
        tokio::spawn(async move {
            send_event(&client_clone, &category).await.expect("Failed to send event");
        })
    });

    futures::future::join_all(futures).await;
    Ok(())
}
