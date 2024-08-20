// network/registry.rs

//! # Registry
//!
//! The `registry` module provides a distributed actor registry using etcd.
//!
//! ## Example
//!
//! ```rust,no_run
//! use astra::network::registry::DistributedRegistry;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    // Skip running the example in tests if the etcd servers are not available
//!    if std::env::var("TEST_ENV").is_err() {
//!        return Ok(()); // Skip test execution unless TEST_ENV is set
//!    }
//!
//!    // Initialize the registry with a list of etcd endpoints
//!    let registry = DistributedRegistry::new(&["http://etcd1:2379", "http://etcd2:2379"]).await?;
//!
//!    // Register an actor with the registry
//!    registry.register_actor("actor1", "http://etcd1:8080").await?;
//!
//!    // Look up the actor's address by its ID
//!    let actor_address = registry.lookup_actor("actor1").await?;
//!
//!    println!("Actor1 is located at: {}", actor_address);
//!    Ok(())
//! }
//! ```

use etcd_client::{Client, GetOptions, PutOptions};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};

pub struct DistributedRegistry {
    client: Arc<Mutex<Client>>,
}

impl DistributedRegistry {
    pub async fn new(endpoints: &[&str]) -> Result<Self, String> {
        let client = timeout(Duration::from_secs(5), Client::connect(endpoints, None))
            .await
            .map_err(|_| "Connection timed out".to_string())?
            .map_err(|e| e.to_string())?;

        Ok(DistributedRegistry {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn register_actor(&self, actor_id: &str, node_address: &str) -> Result<(), String> {
        let mut client = self.client.lock().await;
        client
            .put(actor_id, node_address, Some(PutOptions::new()))
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn lookup_actor(&self, actor_id: &str) -> Result<String, String> {
        let mut client = self.client.lock().await;
        let resp = client
            .get(actor_id, Some(GetOptions::new()))
            .await
            .map_err(|e| e.to_string())?;
        if let Some(kv) = resp.kvs().first() {
            Ok(String::from_utf8(kv.value().to_vec()).unwrap())
        } else {
            Err("Actor not found".to_string())
        }
    }
}
