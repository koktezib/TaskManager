use crate::application::mappers::db_mapper::DbMapper;
use crate::dal::models::TaskEntity;
use crate::domain::task::task::Task;

pub struct TaskDbMapper {}

impl DbMapper<Task, TaskEntity> for TaskDbMapper {
    fn to_db(entity: Task) -> TaskEntity {
        TaskEntity {
            id: entity.id,
            title: entity.title,
            description: entity.description
        }
    }

    fn to_entity(model: TaskEntity) -> Task {
        Task {
            id: model.id,
            title: model.title,
            description: model.description
        }
    }
}
