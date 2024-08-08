// This file is the entry point of the library

pub mod actor_system; // This module is the base system for the actor model
pub mod backends; // This module is to create backends for the data actors
pub mod data_actor; // This module is to create Data Actors
pub mod logging; // This module provides logging utilities
pub mod network; // This module provides different network protocols for the actor system
pub mod snapshot_actor; // This module is to create Snapshot Actors
pub mod supervision; // This module provides supervision strategies for actors
