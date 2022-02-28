extern crate core;

use std::cell::RefCell;
use std::io::ErrorKind;
use std::rc::Rc;

use chrono::{DateTime, Utc};

use todo_rust_api::app::repository::TodoRepository;
use todo_rust_api::app::usecase::{InsertTodo, InsertTodoInput, UpdateTodo, UpdateTodoInput, UseCase};
use todo_rust_api::domain::Todo;
use todo_rust_api::infra::repository::memory::TodoRepositoryMemory;

#[test]
fn should_update_todo() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: RefCell::new(vec![])
    });
    let usecase = UpdateTodo::new(todo_repository.clone());

    let expected_created_date = Some(Utc::now());
    let saved_todo = todo_repository.insert(
        Todo::new(String::from("title"),
                  String::from("description"),
                  expected_created_date)
            .unwrap()
    );
    let saved_todo = saved_todo.unwrap();
    let expected_title = String::from("updated_title");
    let expected_description = String::from("updated_description");
    let expected_updated_date = Some(Utc::now());

    let input = UpdateTodoInput {
        id: saved_todo.id.clone(),
        title: expected_title.clone(),
        description: expected_description.clone(),
        updated_date: expected_updated_date,
    };

    let output = usecase.handle(input);

    if output.is_err() {
        panic!("Output shouldn't be an error!");
    }
    let output = output.unwrap();
    assert_eq!(saved_todo.id, output.id);
    assert_eq!(expected_title, output.title);
    assert_eq!(expected_description, output.description);
    assert_eq!(expected_created_date.unwrap(), output.created_date);
    assert_eq!(expected_updated_date, output.updated_date);

    let repository_todo = todo_repository.get_by_id(output.id.clone());
    if repository_todo.is_err() {
        panic!("Repository todo shouldn't be an error!");
    }
    let repository_todo = repository_todo.unwrap();
    assert_eq!(output.id, repository_todo.id);
    assert_eq!(expected_title, repository_todo.title);
    assert_eq!(expected_description, repository_todo.description);
    assert_eq!(expected_created_date.unwrap(), repository_todo.created_date);
    assert_eq!(expected_updated_date, repository_todo.updated_date);
}

#[test]
fn shouldnt_update_todo_not_found() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: RefCell::new(vec![])
    });
    let usecase = UpdateTodo::new(todo_repository.clone());

    let input = UpdateTodoInput {
        id: String::from("i_am_not_an_id"),
        title: String::from("title"),
        description: String::from("description"),
        updated_date: Some(Utc::now()),
    };

    let output = usecase.handle(input);

    if output.is_ok() {
        panic!("Output shouldn't be Ok!");
    }
    assert_eq!(ErrorKind::NotFound, output.unwrap_err().kind());
}

#[test]
fn shouldnt_update_todo_invalid_input() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: RefCell::new(vec![])
    });
    let usecase = UpdateTodo::new(todo_repository.clone());

    let saved_todo = todo_repository.insert(
        Todo::new(
            String::from("title"),
            String::from("description"),
            None
        ).unwrap()
    );
    let saved_todo = saved_todo.unwrap();

    let input = UpdateTodoInput {
        id: saved_todo.id,
        title: String::new(),
        description: String::new(),
        updated_date: None,
    };

    let output = usecase.handle(input);

    if output.is_ok() {
        panic!("Output shouldn't be Ok!");
    }
    assert_eq!(ErrorKind::InvalidInput, output.unwrap_err().kind());
}