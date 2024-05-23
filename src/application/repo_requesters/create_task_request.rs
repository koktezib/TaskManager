use async_trait::async_trait;
use crate::application::helpers::error_handler::ErrorHandlingHelper;
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
            Err(e) => Err(ErrorHandlingHelper::application_error("Cannot create task", Some(e))),
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
    async fn test_should_create_task_successfully() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        let title = "New Task".to_string();
        let description = Some("This is a new task".to_string());
        let expected_task = Task::new(1, title.clone(), description.clone());
        task_repo.expect_create_task()
            .with(eq(title.clone()), eq(description.clone()))
            .times(1)
            .returning(move |_, _| Ok(expected_task.clone()));

        let create_task_request = CreateTaskRequest::new(title, description, &task_repo);
        let result = create_task_request.execute().await;

        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "New Task");
        assert_eq!(task.description, Some("This is a new task".to_string()));
    }

    #[actix_rt::test]
    async fn test_should_return_error_when_creation_fails() {
        let mut task_repo = MockTaskRepositoryAbstract::new();
        let title = "Failed Task".to_string();
        let description = Some("This task should fail".to_string());
        task_repo.expect_create_task()
            .with(eq(title.clone()), eq(description.clone()))
            .times(1)
            .returning(|_, _| Err(Box::new(Error::new(ErrorKind::Other, "Database error"))));

        let create_task_request = CreateTaskRequest::new(title, description, &task_repo);
        let result = create_task_request.execute().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!("Cannot create task", error.message);
    }
}

