// network/http.rs

use async_trait::async_trait;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Request};

#[async_trait]
pub trait CommunicationProtocol {
    async fn send_message(&self, address: &str, message: &str) -> Result<(), String>;
}

// HTTP implementation
pub struct HttpProtocol;

#[async_trait]
impl CommunicationProtocol for HttpProtocol {
    async fn send_message(&self, address: &str, message: &str) -> Result<(), String> {
        // Create an HTTP connector with default settings
        let connector = HttpConnector::new();

        // Use the connector to create a hyper client
        let client = Client::builder().build::<_, Body>(connector);

        // Create a request using owned message data
        let req = Request::post(address)
            .body(Body::from(message.to_string())) // Convert to owned data
            .map_err(|e| format!("Failed to build request: {}", e))?;

        // Send the request asynchronously
        client
            .request(req)
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        Ok(())
    }
}
