use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};



type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct DbConnection {
    pub db_name: String,
}

impl DbConnection {
    pub fn get_pool(&self) -> DbPool {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
        let pool = r2d2::Pool::builder()
            .max_size(5) // Пример максимального размера пула
            .build(manager)
            .expect("Failed to create pool.");
        pool
    }
}

