use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use deadpool_postgres::{Pool, Client};
use crate::db;



pub async fn status() -> impl Responder {
    "{\"status\": \"UP\"}"
}
pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::get_todo_list(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
