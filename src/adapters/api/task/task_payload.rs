use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Deserialize,Serialize, ToSchema)]
pub struct TaskPayload {
    pub title: String,
    pub description: Option<String>,
}