use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingHelper;
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
            Err(e) => Err(ErrorHandlingHelper::application_error("Task not found", Some(e))),
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
    async fn test_should_update_task_successfully() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        let task_id = 1;
        let title = "Updated Task".to_string();
        let description = Some("Updated description".to_string());
        let expected_task = Task::new(task_id, title.clone(), description.clone());
        task_repo.expect_update_task()
            .with(eq(task_id), eq(title.clone()), eq(description.clone()))
            .times(1)
            .returning(move |_, _, _| Ok(expected_task.clone()));

        let update_task_request = UpdateTaskRequest::new(task_id, title, description, &task_repo);
        let result = update_task_request.execute().await;

        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Updated Task");
        assert_eq!(task.description, Some("Updated description".to_string()));
    }

    #[actix_rt::test]
    async fn test_should_return_error_when_task_does_not_exist() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        let task_id = 2;
        let title = "Nonexistent Task".to_string();
        let description = Some("This task does not exist".to_string());
        task_repo.expect_update_task()
            .with(eq(task_id), eq(title.clone()), eq(description.clone()))
            .times(1)
            .returning(|_, _, _| Err(Box::new(Error::new(ErrorKind::NotFound, "Task not found"))));

        let update_task_request = UpdateTaskRequest::new(task_id, title, description, &task_repo);
        let result = update_task_request.execute().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!("Task not found", error.message);
        assert!(error.error.is_some());
    }
}

