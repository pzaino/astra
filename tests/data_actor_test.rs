use astra::backends::file::FileBackend;
use astra::data_actor::DataActor;
use std::error::Error;

#[tokio::test]
async fn test_data_actor() -> Result<(), Box<dyn Error>> {
    // Initialize the backend with a file called "data.txt"
    let file_backend = FileBackend::new("data.txt").await?;

    // Create a DataActor with the file backend
    let mut actor = DataActor::new(file_backend);

    // Write data to the backend
    actor.write_to_backend("Hello, actor!").await?;

    // Read data from the backend
    let data = actor.read_from_backend().await?;

    // Verify that the written data matches the read data
    assert_eq!(data, "Hello, actor!");

    // Clean up the backend by deleting the file
    actor.cleanup_backend().await?;

    Ok(())
}
