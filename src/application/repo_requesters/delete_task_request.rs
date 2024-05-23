use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingUtils;
use crate::application::repo_requesters::abstract_request::AbstractRequest;
use crate::application::repositories::task_abstract_repo::TaskRepositoryAbstract;
use crate::domain::error::ApiError;

pub struct DeleteTaskRequest<'a> {
    task_id: i32,
    repository: &'a dyn TaskRepositoryAbstract,
}

impl<'a> DeleteTaskRequest<'a> {
    pub fn new(task_id: i32, repository: &'a dyn TaskRepositoryAbstract) -> Self {
        DeleteTaskRequest { task_id, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractRequest<()> for DeleteTaskRequest<'a> {
    async fn execute(&self) -> Result<(), ApiError> {
        let result = self.repository.delete_task(self.task_id).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorHandlingUtils::application_error("task not found", Some(e))),
        }
    }
}
