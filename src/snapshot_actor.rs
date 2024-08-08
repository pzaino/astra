// src/snapshot_actor.rs

//! # Snapshot Actor
//!
//! `SnapshotActor` is an example actor that uses the `DataActor` to save and load its state.
//!
//! # Example
//!
//! ```rust
//! use actor::backends::file::FileBackend;
//! use actor::snapshot_actor::SnapshotActor;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!   let file_backend = FileBackend::new("snapshot.txt").await.unwrap();
//!   let mut actor = SnapshotActor::new("actor1".to_string(), file_backend);
//!   actor.load_state().await.unwrap();
//!   actor.start_snapshot_task().await;
//! }
//! ```

use crate::backends::storage::StorageBackend;
use crate::data_actor::DataActor;
use std::error::Error;
use tokio::time::{interval, Duration};

// Example actor with state snapshots
#[allow(dead_code)]
#[derive(Debug)]
struct SnapshotActor<B: StorageBackend> {
    state: String,
    data_actor: DataActor<B>, // Use DataActor with backend methods
    actor_id: String,
}

impl<B: StorageBackend> SnapshotActor<B> {
    #[allow(dead_code)]
    pub fn new(actor_id: String, backend: B) -> Self {
        let data_actor = DataActor::new(backend); // Use the constructor method
        SnapshotActor {
            state: String::new(),
            data_actor,
            actor_id,
        }
    }

    // Save state using the DataActor's methods
    #[allow(dead_code)]
    async fn save_state(&mut self) -> Result<(), Box<dyn Error>> {
        let data = format!("{}:{}", self.actor_id, self.state); // Format state with actor_id
        self.data_actor.write_to_backend(&data).await?;
        Ok(())
    }

    // Load state using the DataActor's methods
    #[allow(dead_code)]
    async fn load_state(&mut self) -> Result<(), Box<dyn Error>> {
        let data = self.data_actor.read_from_backend().await?;
        let parts: Vec<&str> = data.splitn(2, ':').collect();
        if parts.len() == 2 && parts[0] == self.actor_id {
            self.state = parts[1].to_string();
        }
        Ok(())
    }

    // Start a task to save the state periodically
    #[allow(dead_code)]
    pub async fn start_snapshot_task(&mut self) {
        let mut interval = interval(Duration::from_secs(60)); // Save state every 60 seconds
        loop {
            interval.tick().await;
            if let Err(e) = self.save_state().await {
                eprintln!("Failed to save state: {}", e);
            }
        }
    }
}
