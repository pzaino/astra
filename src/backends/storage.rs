/****************************************************************************************
    Project: ASTRA (Asynchronous Scalable Task and Resource Actors)
    Repository: github.com/pzaino/astra
    Author: Paolo Fabio Zaino
    Copyright: (c) 2023 by Paolo Fabio Zaino, all rights reserved
    License: CDDL Version 1.1
    Check the LICENSE file for more information
    SPDX-License-Identifier: CDDL-1.0
*****************************************************************************************/
// src/backends/storage.rs
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait StorageBackend: Send + Sync + Clone {
    async fn write(&mut self, data: &str) -> Result<(), Box<dyn Error>>;
    async fn read(&mut self) -> Result<String, Box<dyn Error>>;
    async fn cleanup(&mut self) -> Result<(), Box<dyn Error>>;
}
