use crate::model::{TodoList, TodoItem};
use tokio_pg_mapper::FromTokioPostgresRow;
use deadpool_postgres::Client;
use std::io;

pub async fn get_todo_list(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client.prepare("Select * from todo_list").await.unwrap();
    let todos = client.query(&statement, &[]).await
        .expect("Error getting todo lists").iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    Ok(todos)
}