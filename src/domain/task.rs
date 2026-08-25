use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
}

// TODO: add DTO task in storage/dto.rs TaskDto with Serialize/Deserialize
