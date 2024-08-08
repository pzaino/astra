// logging.rs

use async_trait::async_trait;

// Define a trait for logging
#[async_trait]
pub trait Logger {
    async fn log(&self, level: LogLevel, message: &str);
}

// Define log levels
#[derive(Debug)]
pub enum LogLevel {
    Info,
    Error,
    Debug,
}

// Console logger implementation
pub struct ConsoleLogger;

#[async_trait]
impl Logger for ConsoleLogger {
    async fn log(&self, level: LogLevel, message: &str) {
        println!("[{:?}] {}", level, message);
    }
}

// File logger implementation
pub struct FileLogger {
    file_path: String,
}

impl FileLogger {
    pub fn new(file_path: String) -> Self {
        FileLogger { file_path }
    }
}

#[async_trait]
impl Logger for FileLogger {
    async fn log(&self, level: LogLevel, message: &str) {
        let log_message = format!("[{:?}] {}\n", level, message);
        tokio::fs::write(&self.file_path, log_message)
            .await
            .unwrap();
    }
}
