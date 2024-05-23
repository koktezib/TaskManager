use std::env;
use actix_web::dev::Server;
use std::net::TcpListener;

pub mod domain;
pub mod dal;
pub mod adapters;
pub mod infrastructure;
pub mod application;
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    infrastructure::server(listener, &database_name)
}
