extern crate core;

use std::io::ErrorKind;
use std::rc::Rc;
use std::sync::Mutex;

use todo_rust_api::app::repository::TodoRepository;
use todo_rust_api::app::usecase::{DeleteTodoById, DeleteTodoByIdInput, UseCase};
use todo_rust_api::domain::Todo;
use todo_rust_api::infra::repository::memory::TodoRepositoryMemory;

#[test]
fn should_delete_todo_by_id() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: Mutex::new(vec![])
    });
    let usecase = DeleteTodoById::new(todo_repository.clone());

    let todo = Todo::new(
        String::from("title"),
        String::from("description"),
        None,
    );
    let saved_todo = todo_repository.insert(todo.unwrap());
    let saved_todo = saved_todo.unwrap();

    let input = DeleteTodoByIdInput{id: saved_todo.id.clone()};
    let output = usecase.handle(input);

    if output.is_err() {
        panic!("Output shouldn't be an error!");
    }

    let repository_todos = todo_repository.get_all().unwrap();
    assert_eq!(0, repository_todos.len());
}

#[test]
fn shouldnt_delete_todo_by_id() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: Mutex::new(vec![])
    });
    let usecase = DeleteTodoById::new(todo_repository.clone());

    let input = DeleteTodoByIdInput {
        id: String::from("i_am_not_an_id")
    };

    let output = usecase.handle(input);

    if output.is_ok() {
        panic!("Output shouldn't be Ok!");
    }
    assert_eq!(ErrorKind::NotFound, output.unwrap_err().kind());
}