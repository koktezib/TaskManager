use actix_web::web;
use crate::adapters::api::task::task_controller;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api").configure(task_controller::routes));
}
