use std::env;
use std::net::TcpListener;

use task_api::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("0.0.0.0:5535").expect("Failed to bind random port");

    run(listener)?.await
}
