extern crate core;

use std::rc::Rc;
use std::sync::Mutex;

use chrono::Utc;

use todo_rust_api::app::repository::TodoRepository;
use todo_rust_api::app::usecase::{GetAllTodo, GetAllTodoInput, UseCase};
use todo_rust_api::domain::Todo;
use todo_rust_api::infra::repository::memory::TodoRepositoryMemory;

#[test]
fn should_get_all_todo() {
    let todo_repository = Rc::new(TodoRepositoryMemory {
        todos: Mutex::new(vec![])
    });
    let usecase = GetAllTodo::new(todo_repository.clone());

    let expected_title = String::from("title");
    let expected_description = String::from("description");
    let expected_created_date = Some(Utc::now());
    let todo = Todo::new(
        expected_title.clone(), expected_description.clone(), expected_created_date,
    );
    let todo = todo.unwrap();
    let todo2 = Todo::new(
        expected_title.clone(), expected_description.clone(), expected_created_date,
    );
    let _ = todo_repository.insert(todo.clone());
    let _ = todo_repository.insert(todo2.unwrap());

    let output = usecase.handle(GetAllTodoInput {});

    if output.is_err() {
        panic!("Output shouldn't be an error!");
    }
    let output = output.unwrap();
    assert_eq!(output.len(), 2);

    let first = output.get(0).unwrap();
    assert_eq!(todo.id, first.id);
    assert_eq!(todo.title, first.title);
    assert_eq!(todo.description, first.description);
    assert_eq!(todo.created_date, first.created_date);
    assert_eq!(todo.updated_date, first.updated_date);
}