use astra::network::registry::DistributedRegistry;
use std::env;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_registry_with_mock_or_timeout() -> Result<(), Box<dyn std::error::Error>> {
    // Skip test execution unless TEST_ENV is set
    if env::var("TEST_ENV").is_err() {
        return Ok(());
    }

    // Set a timeout to avoid hanging indefinitely
    let registry_result = timeout(
        Duration::from_secs(5),
        DistributedRegistry::new(&["http://etcd1:2379", "http://etcd2:2379"]),
    )
    .await;

    match registry_result {
        Ok(Ok(registry)) => {
            registry
                .register_actor("actor1", "http://etcd1:8080")
                .await?;
            let actor_address = registry.lookup_actor("actor1").await?;
            assert_eq!(actor_address, "http://etcd1:8080");
            Ok(())
        }
        Ok(Err(e)) => {
            eprintln!("Failed to create registry: {:?}", e);
            Err(e.into())
        }
        Err(_) => {
            eprintln!("Timeout while creating registry");
            Err("Timeout occurred".into())
        }
    }
}
