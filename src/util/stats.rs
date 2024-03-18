/*
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatsStore {
    read_event_per_second: i32,
    write_event_per_second: i32,
}

pub fn create_stats_store() -> StatsStore {
    StatsStore {
        read_event_per_second: 0,
        write_event_per_second: 0,
    }
}

pub fn write_stats_store(
    store: &mut StatsStore,
    read_event_per_second: i32,
    write_event_per_second: i32,
) {
    store.read_event_per_second = read_event_per_second;
    store.write_event_per_second = write_event_per_second;
}
*/

/*

In Rust, sharing state between async tasks in a safe manner typically involves using synchronization primitives from the standard library or external crates like tokio. Since Rust enforces strict rules to ensure memory safety and prevent data races, directly sharing mutable state across tasks requires types that ensure safe access, even in concurrent scenarios.

If you want each async task to have its own mutable property that it can write to independently, you can use an Arc<Mutex<T>> (Atomic Reference Counting with Mutex) for shared state or an Arc<RwLock<T>> if you need more granular control (read/write locking). However, giving each task a "separate" mutable property suggests you might instead need a collection of mutable states, each owned by a different task.

Here's a simplified example using tokio::sync::Mutex for async tasks, with each task owning a part of the state independently. We'll simulate this by creating a vector of Mutex<i32>, where each element represents a mutable property that can be independently manipulated by different tasks:

rust
Copy code
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio; // Ensure you're using Tokio 1.x or newer

#[tokio::main]
async fn main() {
    let properties = vec![
        Arc::new(Mutex::new(0)), // Property 1
        Arc::new(Mutex::new(0)), // Property 2
        Arc::new(Mutex::new(0)), // Property 3
    ];

    let mut handles = vec![];

    for i in 0..properties.len() {
        let property = properties[i].clone();
        let handle = tokio::spawn(async move {
            let mut num = property.lock().await;
            *num += i as i32 + 1; // Increment the property by its index + 1 for demonstration
        });
        handles.push(handle);
    }

    // Await all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Check the updated values
    for (i, property) in properties.iter().enumerate() {
        let num = property.lock().await;
        println!("Property {}: {}", i + 1, *num);
    }
}

*/
