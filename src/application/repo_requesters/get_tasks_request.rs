use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingHelper;
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
            Err(e) => Err(ErrorHandlingHelper::application_error("Cannot get all tasks", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use std::io::{Error, ErrorKind};
    use crate::{application::repositories::task_abstract_repo::MockTaskRepositoryAbstract, domain::task::task::Task};

    #[actix_rt::test]
    async fn test_should_return_all_tasks_when_repo_returns_success() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        let expected_tasks = vec![
            Task::new(1, "Complete the project".to_string(), None),
            Task::new(2, "Start the new module".to_string(), Some("Important step".to_string())),
        ];
        task_repo.expect_get_all_tasks()
            .times(1)
            .returning(move || Ok(expected_tasks.clone()));

        let get_all_tasks_request = GetAllTasksRequest::new(&task_repo);
        let result = get_all_tasks_request.execute().await;

        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[0].title, "Complete the project");
        assert!(tasks[0].description.is_none());
        assert_eq!(tasks[1].id, 2);
        assert_eq!(tasks[1].title, "Start the new module");
        assert_eq!(tasks[1].description, Some("Important step".to_string()));
    }

    #[actix_rt::test]
    async fn test_should_return_error_when_repo_returns_error() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        task_repo.expect_get_all_tasks()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "Database error"))));

        let get_all_tasks_request = GetAllTasksRequest::new(&task_repo);
        let result = get_all_tasks_request.execute().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!("Cannot get all tasks", error.message);
    }
}





