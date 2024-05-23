use actix_web::{get, web, HttpResponse, post, put, delete};
use crate::adapters::api::task::error::ErrorReponse;
use crate::adapters::api::task::task_payload::TaskPayload;
use crate::adapters::shared::app_confs::AppConfigs;
use crate::application::repo_requesters::abstract_request::AbstractRequest;
use crate::application::repo_requesters::create_task_request::CreateTaskRequest;
use crate::application::repo_requesters::delete_task_request::DeleteTaskRequest;
use crate::application::repo_requesters::get_task_request::GetTaskRequest;
use crate::application::repo_requesters::get_tasks_request::{GetAllTasksRequest};
use crate::application::repo_requesters::update_task_request::UpdateTaskRequest;
use crate::domain::error::ApiError;
use crate::domain::task::task::Task;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tasks);
    cfg.service(get_task);
    cfg.service(create_task);
    cfg.service(update_task);
    cfg.service(delete_task);

}
#[get("/tasks")]
async fn get_tasks(data: web::Data<AppConfigs>) -> Result<HttpResponse, ErrorReponse> {
    let get_all_tasks_request = GetAllTasksRequest::new(&data.task_repository);
    let tasks: Result<Vec<Task>, ApiError> = get_all_tasks_request.execute().await;

    tasks
        .map_err(ErrorReponse::map_io_error)
        .map(|tasks| HttpResponse::Ok().json(tasks))

}

#[get("/tasks/{id}")]
async fn get_task(data: web::Data<AppConfigs>, task_id: web::Path<i32>) -> Result<HttpResponse, ErrorReponse> {
    let task_id = task_id.into_inner();
    let get_task_by_id = GetTaskRequest::new(&task_id, &data.task_repository);
    let task = get_task_by_id.execute().await;

    task
        .map_err(ErrorReponse::map_io_error)
        .map(|task| HttpResponse::Ok().json(task))
}

#[post("/tasks")]
async fn create_task(data: web::Data<AppConfigs>, input: web::Json<TaskPayload>) -> Result<HttpResponse, ErrorReponse> {
    let TaskPayload { title, description } = input.into_inner();

    let create_task = CreateTaskRequest::new(title, description, &data.task_repository);

    let result = create_task.execute().await;

    match result {
        Ok(task) => Ok(HttpResponse::Ok().json(task)),
        Err(err) => Err(ErrorReponse::map_io_error(err)),
    }
}


#[put("/tasks/{id}")]
async fn update_task(data: web::Data<AppConfigs>, task_id: web::Path<i32>, input: web::Json<TaskPayload>) -> Result<HttpResponse, ErrorReponse> {
    let TaskPayload { title, description } = input.into_inner();

    let update_task_request = UpdateTaskRequest::new(task_id.into_inner(), title, description, &data.task_repository);

    let result = update_task_request.execute().await;

    match result {
        Ok(task) => Ok(HttpResponse::Ok().json(task)),
        Err(err) => Err(ErrorReponse::map_io_error(err)),
    }
}

#[delete("/tasks/{id}")]
async fn delete_task(data: web::Data<AppConfigs>, task_id: web::Path<i32>) -> Result<HttpResponse, ErrorReponse> {
    let delete_task_request = DeleteTaskRequest::new(task_id.into_inner(), &data.task_repository);

    let result = delete_task_request.execute().await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Task deleted successfully")),
        Err(err) => Err(ErrorReponse::map_io_error(err)),
    }
}

