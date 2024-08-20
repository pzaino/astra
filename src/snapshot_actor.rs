// src/snapshot_actor.rs

//! # Snapshot Actor
//!
//! `SnapshotActor` is an example actor that uses the `DataActor` to save and load its state.
//!
//! The actor periodically saves its state using the underlying backend and can be gracefully shut down.
//!
//! # Example
//!
//! ```rust,no_run
//! use astra::backends::file::FileBackend;
//! use astra::snapshot_actor::SnapshotActor;
//! use std::sync::Arc;
//! use tokio::sync::Mutex;
//! use tokio::time::{sleep, Duration};
//!
//! #[tokio::main]
//! async fn main() {
//!   // Initialize the backend and actor
//!   let file_backend = FileBackend::new("snapshot.txt").await.unwrap();
//!   let actor = Arc::new(Mutex::new(SnapshotActor::new("actor1".to_string(), file_backend)));
//!
//!   // Optionally load the actor's previous state from the backend
//!   actor.lock().await.load_state().await.unwrap();
//!
//!   // Set the actor's state
//!   actor.lock().await.set_state("new_state".to_string());
//!
//!   // Start the snapshot task in the background (saving state every 60 seconds)
//!   let actor_clone = Arc::clone(&actor);
//!   let snapshot_task = tokio::spawn(async move {
//!       actor_clone.lock().await.start_snapshot_task().await;
//!   });
//!
//!   // Simulate some work for 5 seconds instead of 2 minutes for the doctest
//!   sleep(Duration::from_secs(5)).await;
//!
//!   // Send a shutdown signal to stop the snapshot task
//!   actor.lock().await.shutdown();
//!
//!   // Wait for the snapshot task to finish
//!   snapshot_task.await.unwrap();
//!
//!   // Optionally save the final state and verify it
//!   actor.lock().await.save_state().await.unwrap();
//!   println!("Final actor state: {}", actor.lock().await.get_state());
//! }
//! ```

use crate::backends::storage::StorageBackend;
use crate::data_actor::DataActor;
use std::error::Error;
use tokio::sync::watch;
use tokio::time::{interval, Duration};

#[derive(Debug, Clone)]
pub struct SnapshotActor<B: StorageBackend> {
    state: String,
    data_actor: DataActor<B>,
    actor_id: String,
    shutdown_tx: watch::Sender<()>,
    shutdown_rx: watch::Receiver<()>,
}

impl<B: StorageBackend> SnapshotActor<B> {
    pub fn new(actor_id: String, backend: B) -> Self {
        let data_actor = DataActor::new(backend);
        let (shutdown_tx, shutdown_rx) = watch::channel(());

        SnapshotActor {
            state: String::new(),
            data_actor,
            actor_id,
            shutdown_tx,
            shutdown_rx,
        }
    }

    // Save state using the DataActor's methods
    pub async fn save_state(&mut self) -> Result<(), Box<dyn Error>> {
        let data = format!("{}:{}", self.actor_id, self.state);
        self.data_actor.write_to_backend(&data).await?;
        Ok(())
    }

    // Load state using the DataActor's methods
    pub async fn load_state(&mut self) -> Result<(), Box<dyn Error>> {
        let data = self.data_actor.read_from_backend().await?;
        let parts: Vec<&str> = data.splitn(2, ':').collect();
        if parts.len() == 2 && parts[0] == self.actor_id {
            self.state = parts[1].to_string();
        }
        Ok(())
    }

    // Method to set the state
    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    // Get the current state
    pub fn get_state(&self) -> String {
        self.state.clone()
    }

    // Start a task to save the state periodically
    pub async fn start_snapshot_task(&mut self) {
        let mut interval = interval(Duration::from_secs(60)); // Save state every 60 seconds
        let mut shutdown_rx = self.shutdown_rx.clone(); // Clone receiver for the task

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Err(e) = self.save_state().await {
                        eprintln!("Failed to save state: {}", e);
                    }
                },
                _ = shutdown_rx.changed() => {
                    println!("Received shutdown signal, stopping snapshot task");
                    break;
                }
            }
        }
    }

    // Send a shutdown signal to stop the snapshot task
    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }
}
