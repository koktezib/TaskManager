use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingHelper;
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
            Err(e) => Err(ErrorHandlingHelper::application_error("task not found", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use std::io::{Error, ErrorKind};
    use crate::{application::repositories::task_abstract_repo::MockTaskRepositoryAbstract, domain::error::ApiError};

    #[actix_rt::test]
    async fn test_should_delete_task_successfully() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        let task_id = 1;
        task_repo.expect_delete_task()
            .with(eq(task_id))
            .times(1)
            .returning(move |_| Ok(()));

        let delete_task_request = DeleteTaskRequest::new(task_id, &task_repo);
        let result = delete_task_request.execute().await;

        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_should_return_error_when_task_does_not_exist() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        let task_id = 2;
        task_repo.expect_delete_task()
            .with(eq(task_id))
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::NotFound, "Task not found"))));

        let delete_task_request = DeleteTaskRequest::new(task_id, &task_repo);
        let result = delete_task_request.execute().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!("task not found", error.message);
        assert!(error.error.is_some());
    }
}

