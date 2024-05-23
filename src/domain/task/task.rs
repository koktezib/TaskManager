use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone,Serialize,Deserialize, ToSchema)]
pub struct Task
{
    pub id: i32,
    pub title: String,
    pub description: Option<String>
}

impl Task
{
    pub fn new(id: i32, title: String, description: Option<String>) -> Self
    {
        Task {id,title,description}
    }
}