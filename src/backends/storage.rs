// src/backends/storage.rs
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn write(&mut self, data: &str) -> Result<(), Box<dyn Error>>;
    async fn read(&mut self) -> Result<String, Box<dyn Error>>;
    async fn cleanup(&mut self) -> Result<(), Box<dyn Error>>;
}
