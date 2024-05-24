use std::{env, net::TcpListener};
use actix_web::{dev::Server};
use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::dal::db_connection::DbConnection;
use crate::dal::repositories::db_repo_tasks::TaskRepository;
use crate::adapters::routes;
use crate::adapters::shared::app_confs::AppConfigs;
use crate::adapters::api::task::task_payload::TaskPayload;
use crate::adapters::api::task::task_controller::{__path_get_tasks,__path_create_task,__path_get_task,__path_delete_task,__path_update_task};
use crate::domain::task::task::Task;
#[derive(OpenApi)]
#[openapi(
paths(
get_tasks,
get_task,
create_task,
update_task,
delete_task,
),
components(schemas(TaskPayload,Task))
)]


struct ApiDoc;
/// Инициализирует и запускает HTTP сервер на основе переданного слушателя сокетов.
///
/// # Параметры
/// - `listener`: Слушатель для входящих соединений.
/// - `db_name`: Имя базы данных для инициализации соединения.
///
/// # Возвращаемое значение
/// Возвращает экземпляр `Server`, который может быть запущен асинхронно.
///
/// # Ошибки
/// Возвращает `std::io::Error`, если сервер не может быть запущен.
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
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .configure(routes::routes)
    })
        .listen(listener)?
        .run();

    println!("Server running on port {}, DB_NAME {}", port, db_name);

    Ok(server)
}