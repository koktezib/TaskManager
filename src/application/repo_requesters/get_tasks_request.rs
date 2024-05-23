use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingUtils;
use crate::application::repo_requesters::abstract_request::AbstractRequest;
use crate::application::repositories::task_abstract_repo::TaskRepositoryAbstract;
use crate::domain::error::ApiError;
use crate::domain::task::task::Task;

pub struct GetAllTasksRequest<'a> {
    repository: &'a dyn TaskRepositoryAbstract,
}

impl<'a> GetAllTasksRequest<'a> {
    pub fn new(repository: &'a dyn TaskRepositoryAbstract) -> Self {
        GetAllTasksRequest { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractRequest<Vec<Task>> for GetAllTasksRequest<'a> {
    async fn execute(&self) -> Result<Vec<Task>, ApiError> {
        let tasks = self.repository.get_all_tasks().await;

        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all tasks", Some(e))),
        }
    }
}
