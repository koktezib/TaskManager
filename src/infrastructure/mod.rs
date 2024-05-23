use std::{env, net::TcpListener};
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use crate::dal::db_connection::DbConnection;
use crate::dal::repositories::db_repo_tasks::TaskRepository;
use crate::adapters::routes;
use crate::adapters::shared::app_confs::AppConfigs;

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_LOG", "diesel=debug,r2d2=debug");


    env_logger::try_init().expect("Logger not init");

    let db_connection: DbConnection = DbConnection { db_name: db_name.to_string() };

    let data = web::Data::new(AppConfigs {
        task_repository: TaskRepository { db_connection },
    });

    let port = listener.local_addr().unwrap().port();


    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(routes::routes)
    })
        .listen(listener)?
        .run();

    println!("Server running on port {}, DB_NAME {}", port, db_name);

    Ok(server)
}