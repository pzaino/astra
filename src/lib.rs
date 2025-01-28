/****************************************************************************************
    Project: ASTRA (Asynchronous Scalable Task and Resource Actors)
    Repository: github.com/pzaino/astra
    Author: Paolo Fabio Zaino
    Copyright: (c) 2023 by Paolo Fabio Zaino, all rights reserved
    License: CDDL Version 1.1
    Check the LICENSE file for more information
    SPDX-License-Identifier: CDDL-1.0
*****************************************************************************************/

// This file is the entry point of the library

pub mod actor_system; // This module is the base system for the actor model
pub mod backends; // This module is to create backends for the data actors
pub mod data_actor; // This module is to create Data Actors
pub mod logging; // This module provides logging utilities
pub mod network; // This module provides different network protocols for the actor system
pub mod snapshot_actor; // This module is to create Snapshot Actors
pub mod supervision; // This module provides supervision strategies for actors
