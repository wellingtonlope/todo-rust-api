extern crate core;

use std::cell::RefCell;
use std::io::ErrorKind;
use std::rc::Rc;

use chrono::Utc;

use todo_rust_api::app::repository::TodoRepository;
use todo_rust_api::app::usecase::{InsertTodo, InsertTodoInput, UseCase};
use todo_rust_api::infra::repository::memory::TodoRepositoryMemory;

#[test]
fn should_insert_todo() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: RefCell::new(vec![])
    });
    let usecase = InsertTodo::new(todo_repository.clone());

    let expected_title = String::from("title");
    let expected_description = String::from("description");
    let expected_created_date = Some(Utc::now());
    let input = InsertTodoInput {
        title: expected_title.clone(),
        description: expected_description.clone(),
        created_date: expected_created_date,
    };

    let output = usecase.handle(input);

    if output.is_err() {
        panic!("Output shouldn't be an error!");
    }
    let output = output.unwrap();
    assert_ne!(String::new(), output.id);
    assert_eq!(expected_title, output.title);
    assert_eq!(expected_description, output.description);
    assert_eq!(expected_created_date.unwrap(), output.created_date);

    let saved_todo = todo_repository.get_by_id(output.id.clone());
    if saved_todo.is_err() {
        panic!("Saved todo shouldn't be an error!");
    }
    let saved_todo = saved_todo.unwrap();
    assert_eq!(output.id, saved_todo.id);
    assert_eq!(expected_title, saved_todo.title);
    assert_eq!(expected_description, saved_todo.description);
    assert_eq!(expected_created_date.unwrap(), saved_todo.created_date);
}

#[test]
fn shouldnt_insert_todo() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: RefCell::new(vec![])
    });
    let usecase = InsertTodo::new(todo_repository);

    let input = InsertTodoInput {
        title: String::new(),
        description: String::new(),
        created_date: None,
    };

    let output = usecase.handle(input);

    if output.is_ok() {
        panic!("Output shouldn't be Ok!");
    }
    assert_eq!(ErrorKind::InvalidInput, output.unwrap_err().kind());
}