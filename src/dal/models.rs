use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use serde::{Deserialize, Serialize};
use crate::dal::schema::tasks;

#[derive(Insertable, Selectable,QueryableByName,Queryable, Serialize, Deserialize, Debug)]
#[table_name = "tasks"]
pub struct TaskEntity {
    pub id: i32,
    pub title: String,
    pub description: Option<String>
}
