// src/actor_system.rs

//! # Asynchronous Actor System
//!
//! The `actor` module provides an asynchronous actor system.
//!
//! ## Example
//!
//! ```rust
//! use astra::actor_system::{Actor, ActorSystem, Message};
//! use async_trait::async_trait;
//! use std::error::Error;
//!
//! struct SimpleActor;
//!
//! #[async_trait]
//! impl Actor for SimpleActor {
//!     type Message = String;
//!     type Error = String;
//!
//!     async fn receive(&mut self, message: Message<Self::Message>) -> Result<(), Self::Error> {
//!         match message {
//!             Message::Regular(msg) => {
//!                 println!("Received message: {}", msg);
//!                 Ok(())
//!             }
//!             Message::Shutdown => {
//!                 println!("Shutting down SimpleActor.");
//!                 Ok(())
//!             }
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let mut system = ActorSystem::new();
//!     system.add_actor("simple_actor".to_string(), SimpleActor);
//!     system.send_message("simple_actor", "Hello, actor!".to_string()).await?;
//!     system.shutdown().await;
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task;

/// The `Actor` trait defines the interface for any actor within the actor system.
/// Implementors of this trait are responsible for processing messages and managing their resources.
#[async_trait]
pub trait Actor {
    /// The type of messages the actor can receive.
    type Message: std::fmt::Debug;

    /// The type of errors that can occur when processing a message.
    type Error: std::fmt::Debug;

    /// Processes a message. Implementors should define the logic for handling different messages here.
    async fn receive(&mut self, message: Message<Self::Message>) -> Result<(), Self::Error>;

    /// Cleans up resources used by the actor. This method is called when the actor system shuts down.
    async fn cleanup(&mut self) {
        // Default cleanup implementation
    }
}

#[derive(Debug, Clone)]
pub enum Message<M> {
    Regular(M),
    Shutdown,
}

#[derive(Debug, Clone)]
pub struct ActorSystem<M> {
    actors: HashMap<String, Sender<Message<M>>>,
}

impl<M: Send + 'static + std::fmt::Debug> ActorSystem<M> {
    pub fn new() -> Self {
        ActorSystem {
            actors: HashMap::new(),
        }
    }

    pub fn add_actor<A>(&mut self, name: String, mut actor: A)
    where
        A: Actor<Message = M, Error = String> + Send + 'static,
        M: std::fmt::Debug,
    {
        let (tx, mut rx): (Sender<Message<M>>, Receiver<Message<M>>) = mpsc::channel(100);

        task::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = actor.receive(message).await {
                    println!("Error processing message: {:?}", e);
                }
            }
            actor.cleanup().await;
        });

        self.actors.insert(name, tx);
    }

    pub async fn send_message(&self, actor_name: &str, message: M) -> Result<(), String> {
        if let Some(actor) = self.actors.get(actor_name) {
            actor
                .send(Message::Regular(message))
                .await
                .map_err(|e| format!("Failed to send message: {:?}", e))
        } else {
            Err(format!("Actor {} not found", actor_name))
        }
    }

    pub async fn shutdown(&self) {
        for (name, sender) in &self.actors {
            if let Err(e) = sender.send(Message::Shutdown).await {
                println!("Failed to send shutdown signal to actor {}: {:?}", name, e);
            }
        }
    }
}
