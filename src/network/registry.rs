// network/registry.rs

//! # Registry
//!
//! The `registry` module provides a distributed actor registry using etcd.
//!
//! ## Example
//!
//! ```rust
//! use network::registry::DistributedRegistry;
//!
//! #[tokio::main]
//! async fn main() {
//!    let registry = DistributedRegistry::new(&["http://etcd1:2379", "http://etcd2:2379"]).await;
//!   registry.register_actor("actor1", "http://etcd1:8080").await;
//!   let actor_address = registry.lookup_actor("actor1").await;
//! }
//! ```
//!

use etcd_client::{Client, GetOptions, PutOptions};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DistributedRegistry {
    client: Arc<Mutex<Client>>,
}

impl DistributedRegistry {
    pub async fn new(endpoints: &[&str]) -> Self {
        let client = Client::connect(endpoints, None)
            .await
            .expect("Failed to connect to etcd");
        DistributedRegistry {
            client: Arc::new(Mutex::new(client)),
        }
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
