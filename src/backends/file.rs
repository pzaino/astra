// src/backends/file.rs

use super::storage::StorageBackend;
use async_trait::async_trait;
use std::error::Error;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub struct FileBackend {
    file: File,
}

impl FileBackend {
    pub async fn new(file_path: &str) -> io::Result<Self> {
        let file = File::create(file_path).await?;
        Ok(FileBackend { file })
    }
}

#[async_trait]
impl StorageBackend for FileBackend {
    // Write data to the file
    async fn write(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        self.file.write_all(data.as_bytes()).await?;
        Ok(())
    }

    // Read the contents of the file
    async fn read(&mut self) -> Result<String, Box<dyn Error>> {
        let mut content = String::new();
        self.file.read_to_string(&mut content).await?;
        Ok(content)
    }

    // Sync the file to ensure all data is written to disk
    async fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        self.file.sync_all().await?;
        Ok(())
    }
}
