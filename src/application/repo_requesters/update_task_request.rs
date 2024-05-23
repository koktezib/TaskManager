use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingUtils;
use crate::application::repo_requesters::abstract_request::AbstractRequest;
use crate::application::repositories::task_abstract_repo::TaskRepositoryAbstract;
use crate::domain::error::ApiError;
use crate::domain::task::task::Task;

pub struct UpdateTaskRequest<'a> {
    task_id: i32,
    title: String,
    description: Option<String>,
    repository: &'a dyn TaskRepositoryAbstract,
}

impl<'a> UpdateTaskRequest<'a> {
    pub fn new(task_id: i32, title: String, description: Option<String>, repository: &'a dyn TaskRepositoryAbstract) -> Self {
        UpdateTaskRequest { task_id, title, description, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractRequest<Task> for UpdateTaskRequest<'a> {
    async fn execute(&self) -> Result<Task, ApiError> {
        let task = self.repository.update_task(self.task_id, self.title.clone(), self.description.clone()).await;

        match task {
            Ok(task) => Ok(task),
            Err(e) => Err(ErrorHandlingUtils::application_error("Task not found", Some(e))),
        }
    }
}
