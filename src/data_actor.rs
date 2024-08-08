// src/data_actor.rs
use crate::backends::storage::StorageBackend;
use async_trait::async_trait;
use std::error::Error;
//use std::fmt::Debug;

use crate::actor_system::{Actor, Message}; // Assuming Actor and Message are defined in a module named actor_system

/// `DataActor` is a generic actor that can perform operations using a specified backend.
///
/// # Example
///
/// ```
/// use your_project::actor::DataActor;
/// use your_project::backends::file_backend::FileBackend;
///
/// #[tokio::main]
/// async fn main() {
///     let file_backend = FileBackend::new("example.txt").await.unwrap();
///     let actor = DataActor { backend: file_backend };
/// }
/// ```
#[allow(dead_code)]
#[derive(Debug)]
pub struct DataActor<B: StorageBackend> {
    backend: B,
}

#[async_trait]
impl<B: StorageBackend + 'static> Actor for DataActor<B> {
    type Message = String;
    type Error = Box<dyn Error>;

    async fn receive(&mut self, message: Message<Self::Message>) -> Result<(), Self::Error> {
        match message {
            Message::Regular(data) => {
                self.backend.write(&data).await?;
                Ok(())
            }
            Message::Shutdown => {
                println!("Shutting down DataActor.");
                self.backend.cleanup().await?;
                Ok(())
            }
        }
    }

    async fn cleanup(&mut self) {
        if let Err(e) = self.backend.cleanup().await {
            println!("Failed to clean up backend: {:?}", e);
        }
    }
}

impl<B: StorageBackend> DataActor<B> {
    /// Creates a new `DataActor` with the given backend.
    pub fn new(backend: B) -> Self {
        DataActor { backend }
    }

    /// Writes data to the backend.
    pub async fn write_to_backend(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        self.backend.write(data).await
    }

    /// Reads data from the backend.
    pub async fn read_from_backend(&mut self) -> Result<String, Box<dyn Error>> {
        self.backend.read().await
    }

    /// Cleans up the backend.
    pub async fn cleanup_backend(&mut self) -> Result<(), Box<dyn Error>> {
        self.backend.cleanup().await
    }
}
