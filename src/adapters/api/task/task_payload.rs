use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct TaskPayload {
    pub title: String,
    pub description: Option<String>,
}