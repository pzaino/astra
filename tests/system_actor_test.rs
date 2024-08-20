use astra::actor_system::{Actor, ActorSystem, Message};
use async_trait::async_trait;
use std::error::Error;

struct SimpleActor;

#[async_trait]
impl Actor for SimpleActor {
    type Message = String;
    type Error = String;

    async fn receive(&mut self, message: Message<Self::Message>) -> Result<(), Self::Error> {
        match message {
            Message::Regular(msg) => {
                println!("Received message: {}", msg);
                Ok(())
            }
            Message::Shutdown => {
                println!("Shutting down SimpleActor.");
                Ok(())
            }
        }
    }
}

#[tokio::test]
async fn test_actor_system() -> Result<(), Box<dyn Error>> {
    // Initialize the actor system and add a SimpleActor
    let mut system = ActorSystem::new();
    system.add_actor("simple_actor".to_string(), SimpleActor);

    // Send a message to the actor and verify it processes correctly
    system
        .send_message("simple_actor", "Hello, actor!".to_string())
        .await?;

    // Shutdown the system and ensure proper cleanup
    system.shutdown().await;

    Ok(())
}
