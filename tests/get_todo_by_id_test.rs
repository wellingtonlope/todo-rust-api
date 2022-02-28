extern crate core;

use std::cell::RefCell;
use std::io::ErrorKind;
use std::rc::Rc;

use chrono::Utc;

use todo_rust_api::app::repository::TodoRepository;
use todo_rust_api::app::usecase::{GetTodoById, GetTodoByIdInput, UseCase};
use todo_rust_api::domain::Todo;
use todo_rust_api::infra::repository::memory::TodoRepositoryMemory;

#[test]
fn should_get_todo_by_id() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: RefCell::new(vec![])
    });
    let usecase = GetTodoById::new(todo_repository.clone());

    let expected_title = String::from("title");
    let expected_description = String::from("description");
    let expected_created_date = Some(Utc::now());
    let todo = Todo::new(
        expected_title.clone(), expected_description.clone(), expected_created_date,
    );
    let saved_todo = todo_repository.insert(todo.unwrap());
    let saved_todo = saved_todo.unwrap();

    let input = GetTodoByIdInput{id: saved_todo.id.clone()};
    let output = usecase.handle(input);

    if output.is_err() {
        panic!("Output shouldn't be an error!");
    }
    let output = output.unwrap();
    assert_eq!(saved_todo.id, output.id);
    assert_eq!(saved_todo.title, output.title);
    assert_eq!(saved_todo.description, output.description);
    assert_eq!(saved_todo.created_date, output.created_date);
}

#[test]
fn shouldnt_get_todo_by_id() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: RefCell::new(vec![])
    });
    let usecase = GetTodoById::new(todo_repository.clone());

    let input = GetTodoByIdInput {
        id: String::from("i_am_not_an_id")
    };

    let output = usecase.handle(input);

    if output.is_ok() {
        panic!("Output shouldn't be Ok!");
    }
    assert_eq!(ErrorKind::NotFound, output.unwrap_err().kind());
}