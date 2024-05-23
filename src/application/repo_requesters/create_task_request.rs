use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingUtils;
use crate::application::repo_requesters::abstract_request::AbstractRequest;
use crate::application::repositories::task_abstract_repo::TaskRepositoryAbstract;
use crate::domain::error::ApiError;
use crate::domain::task::task::Task;

pub struct CreateTaskRequest<'a> {
    title: String,
    description: Option<String>,
    repository: &'a dyn TaskRepositoryAbstract,
}

impl<'a> CreateTaskRequest<'a> {
    pub fn new(title: String, description: Option<String>, repository: &'a dyn TaskRepositoryAbstract) -> Self {
        CreateTaskRequest { title, description, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractRequest<Task> for CreateTaskRequest<'a> {
    async fn execute(&self) -> Result<Task, ApiError> {
        let task = self.repository.create_task(self.title.clone(), self.description.clone()).await;

        match task {
            Ok(task) => Ok(task),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot create task", Some(e))),
        }
    }
}
