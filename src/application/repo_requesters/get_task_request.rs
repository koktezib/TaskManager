use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingUtils;
use crate::application::repo_requesters::abstract_request::AbstractRequest;
use crate::application::repositories::task_abstract_repo::TaskRepositoryAbstract;
use crate::domain::error::ApiError;
use crate::domain::task::task::Task;

pub struct GetTaskRequest<'a> {
    task_id: &'a i32,
    repository: &'a dyn TaskRepositoryAbstract,
}

impl<'a> GetTaskRequest<'a> {
    pub fn new(task_id: &'a i32, repository: &'a dyn TaskRepositoryAbstract) -> Self {
        GetTaskRequest { task_id, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractRequest<Task> for GetTaskRequest<'a> {
    async fn execute(&self) -> Result<Task, ApiError> {
        let task = self.repository.get_task_by_id(*self.task_id).await;

        match task {
            Ok(task) => Ok(task),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get single task", Some(e))),
        }
    }
}
