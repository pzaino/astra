/****************************************************************************************
    Project: ASTRA (Asynchronous Scalable Task and Resource Actors)
    Repository: github.com/pzaino/astra
    Author: Paolo Fabio Zaino
    Copyright: (c) 2023 by Paolo Fabio Zaino, all rights reserved
    License: CDDL Version 1.1
    Check the LICENSE file for more information
    SPDX-License-Identifier: CDDL-1.0
*****************************************************************************************/

// supervision.rs

pub struct Supervisor {
    strategy: SupervisionStrategy,
}

pub enum SupervisionStrategy {
    Restart,
    Ignore,
    Escalate,
}

impl Supervisor {
    pub fn new(strategy: SupervisionStrategy) -> Self {
        Supervisor { strategy }
    }

    pub fn handle_failure(&self, actor_name: &str, error: &str) {
        match self.strategy {
            SupervisionStrategy::Restart => {
                println!("Restarting actor {} due to error: {}", actor_name, error);
                // Logic to restart the actor
            }
            SupervisionStrategy::Ignore => {
                println!("Ignoring error for actor {}: {}", actor_name, error);
            }
            SupervisionStrategy::Escalate => {
                println!("Escalating error for actor {}: {}", actor_name, error);
                // Logic to escalate the error
            }
        }
    }
}
