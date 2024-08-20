// src/backends/file.rs

use super::storage::StorageBackend;
use async_trait::async_trait;
use std::error::Error;
use tokio::fs::{self, File};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone)]
pub struct FileBackend {
    file_path: String,
}

impl FileBackend {
    // Create a new FileBackend with the given file path
    pub async fn new(file_path: &str) -> io::Result<Self> {
        // Ensure that the file exists or is created
        File::create(file_path).await?;
        Ok(FileBackend {
            file_path: file_path.to_string(),
        })
    }
}

#[async_trait]
impl StorageBackend for FileBackend {
    // Write data to the file
    async fn write(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        // Open the file for writing and write data
        let mut file = File::create(&self.file_path).await?;
        file.write_all(data.as_bytes()).await?;
        Ok(())
    }

    // Read the contents of the file
    async fn read(&mut self) -> Result<String, Box<dyn Error>> {
        // Open the file for reading and read its content
        let mut file = File::open(&self.file_path).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        Ok(content)
    }

    // Clean up by deleting the file
    async fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        fs::remove_file(&self.file_path).await?;
        Ok(())
    }
}
