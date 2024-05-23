use async_trait::async_trait;
use crate::dal::models::TaskEntity;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;
use crate::domain::task::task::Task;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait TaskRepositoryAbstract {
    async fn get_all_tasks(&self) -> Result<Vec<Task>, Box<dyn Error>>;
    async fn get_task_by_id(&self, task_id: i32) -> Result<Task, Box<dyn Error>>;

    async fn create_task(&self, title: String, description: Option<String>) -> Result<Task, Box<dyn Error>>;
    async fn update_task(&self, task_id: i32, title: String, description: Option<String>) -> Result<Task, Box<dyn Error>>;
    async fn delete_task(&self, task_id: i32) -> Result<(), Box<dyn Error>>;

}

