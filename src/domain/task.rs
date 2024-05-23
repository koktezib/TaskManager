#[derive(Debug, Clone)]
pub struct Task
{
    pub title: String,
    pub description: Option<String>
}

impl Task
{
    pub fn new(title: String, description: Option<String>) -> Self
    {
        Task {title,description}
    }
}