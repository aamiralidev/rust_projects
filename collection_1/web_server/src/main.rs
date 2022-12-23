mod config;
mod model;
mod db;
mod handler;

use handler::status;
use handler::get_todos;
use tokio_postgres::NoTls;
use dotenv::dotenv;
#[allow(non_snake_case)]
use std::io::Result;
use actix_web::{HttpServer, web, App};

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    println!("Server Started at http://{}:{}", config.server.host, config.server.port);
    HttpServer::new(
        move || {
            App::new().data(pool.clone())
                .route("/", web::get().to(status))
                .route("/todos{_:/?}", web::get().to(get_todos))
        }).bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await;

    Ok(())
}
