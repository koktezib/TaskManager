use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use diesel::{delete, update};
use crate::application::mappers::db_mapper::DbMapper;
use crate::dal::db_connection::DbConnection;
use crate::application::repositories::task_abstract_repo::TaskRepositoryAbstract;
use crate::dal::db_mapper::TaskDbMapper;
use crate::dal::models::{TaskEntity};
use crate::domain::task::task::Task;
use crate::dal::schema::tasks::dsl::*;

pub struct TaskRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl TaskRepositoryAbstract for TaskRepository {


    async fn get_all_tasks(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let results = tasks.load::<TaskEntity>(&mut conn);

        match results {
            Ok(models) => Ok(models.into_iter().map(TaskDbMapper::to_entity).collect::<Vec<Task>>()),
        Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete_task(&self, task_id: i32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let task = tasks.filter(id.eq(task_id)).first::<TaskEntity>(&mut conn).optional();

        match task {
            Ok(Some(_)) => {
                let result = delete(tasks.filter(id.eq(task_id))).execute(&mut conn);
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::new(e)),
                }
            },
            Ok(None) => Err(Box::new(diesel::result::Error::NotFound)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_task_by_id(&self, task_id: i32) -> Result<Task, Box<dyn Error>> {
        let mut  conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let result = tasks.filter(id.eq(task_id)).get_result::<TaskEntity>(&mut conn);

        match result {
            Ok(model) => Ok(TaskDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }

    }

    async fn create_task(&self, new_title: String, new_description: Option<String>) -> Result<Task, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let result = diesel::insert_into(tasks)
            .values((title.eq(new_title), description.eq(new_description)))
            .execute(&mut conn);

        match result {
            Ok(_) => {
                let inserted_task = tasks.order(id.desc()).first::<TaskEntity>(&mut conn);
                match inserted_task {
                    Ok(model) => Ok(TaskDbMapper::to_entity(model)),
                    Err(e) => Err(Box::new(e)),
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_task(&self, task_id: i32, new_title: String, new_description: Option<String>) -> Result<Task, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let result = update(tasks.filter(id.eq(task_id)))
            .set((
                title.eq(new_title),
                description.eq(new_description),
            ))
            .execute(&mut conn);

        match result {
            Ok(_) => {
                let updated_task = tasks.filter(id.eq(task_id)).first::<TaskEntity>(&mut conn);
                match updated_task {
                    Ok(model) => Ok(TaskDbMapper::to_entity(model)),
                    Err(e) => Err(Box::new(e)),
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }
    /*


        async fn update_task(&self, task_id: i32, title: Option<String>, description: Option<String>) -> Result<Task, String> {
            Err("fe".to_string())

        }

        async fn delete_task(&self, task_id: i32) -> Result<(), String> {
            Err("fe".to_string())
        }

     */
}
