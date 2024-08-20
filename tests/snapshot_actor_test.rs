use astra::backends::file::FileBackend;
use astra::snapshot_actor::SnapshotActor;
use std::error::Error;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_snapshot_actor_lifecycle() -> Result<(), Box<dyn Error>> {
    let file_backend = FileBackend::new("snapshot_test.txt").await?;
    let mut actor = SnapshotActor::new("actor1".to_string(), file_backend);

    // Set the actor's state to a specific value
    actor.set_state("expected_state".to_string());

    // Load initial state
    actor.load_state().await?;

    // Start the snapshot task in the background
    let mut actor_clone = actor.clone();
    let snapshot_task = tokio::spawn(async move {
        actor_clone.start_snapshot_task().await;
    });

    // Simulate some work
    sleep(Duration::from_secs(5)).await;

    // Shutdown the snapshot task
    actor.shutdown();
    snapshot_task.await.unwrap();

    // Save and load state again
    actor.save_state().await?;
    actor.load_state().await?;

    // Assert that the state matches the expected state
    assert_eq!(actor.get_state(), "expected_state");

    Ok(())
}
