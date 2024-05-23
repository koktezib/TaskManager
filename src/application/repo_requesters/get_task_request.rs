use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingHelper;
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
            Err(e) => Err(ErrorHandlingHelper::application_error("Cannot get task", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use std::io::{Error, ErrorKind};
    use crate::{application::repositories::task_abstract_repo::MockTaskRepositoryAbstract, domain::task::task::Task};

    #[actix_rt::test]
    async fn test_should_return_task_when_repo_returns_success() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        let expected_task = Task::new(1, "Complete the project".to_string(), None);
        task_repo
            .expect_get_task_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |_| Ok(expected_task.clone()));

        let get_one_task_by_id = GetTaskRequest::new(&1, &task_repo);
        let result = get_one_task_by_id.execute().await;

        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Complete the project");
        assert!(task.description.is_none());
    }

    #[actix_rt::test]
    async fn test_should_return_error_when_invalid_id_provided() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        task_repo
            .expect_get_task_by_id()
            .with(eq(-1))
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::InvalidInput, "Invalid ID"))));

        let get_one_task_by_id = GetTaskRequest::new(&-1, &task_repo);
        let result = get_one_task_by_id.execute().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.message, "Invalid ID provided");
    }

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        task_repo
            .expect_get_task_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        let get_one_task_by_id = GetTaskRequest::new(&1, &task_repo);
        let data = get_one_task_by_id.execute().await;

        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get task", result.message);
    }
}


