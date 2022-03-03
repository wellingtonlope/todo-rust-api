use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use serde::{Deserialize, Serialize};

use todo_rust_api::app::usecase::{DeleteTodoById, GetAllTodo, GetTodoById, InsertTodo, UpdateTodo};
use todo_rust_api::infra::http;
use todo_rust_api::infra::repository::memory::TodoRepositoryMemory;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_repository = Arc::new(TodoRepositoryMemory {
        todos: Mutex::new(vec![])
    });
    let get_todo_by_id = GetTodoById::new(todo_repository.clone());
    let insert_todo = InsertTodo::new(todo_repository.clone());
    let get_all_todo = GetAllTodo::new(todo_repository.clone());
    let update_todo = UpdateTodo::new(todo_repository.clone());
    let delete_todo_by_id = DeleteTodoById::new(todo_repository.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(get_todo_by_id.clone()))
            .app_data(web::Data::new(insert_todo.clone()))
            .app_data(web::Data::new(get_all_todo.clone()))
            .app_data(web::Data::new(update_todo.clone()))
            .app_data(web::Data::new(delete_todo_by_id.clone()))
            .route("/todo/{id}", web::get().to(http::todo::get_todo_by_id))
            .route("/todo", web::post().to(http::todo::insert_todo))
            .route("/todo", web::get().to(http::todo::get_all_todo))
            .route("/todo/{id}", web::put().to(http::todo::update_todo))
            .route("/todo/{id}", web::delete().to(http::todo::delete_todo_by_id))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
