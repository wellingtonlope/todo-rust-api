use std::cell::RefCell;
use std::io::{Error, ErrorKind, Result};

use crate::app::repository::TodoRepository;
use crate::domain::Todo;

pub struct TodoRepositoryMemory {
    pub todos: RefCell<Vec<Todo>>,
}

impl TodoRepository for TodoRepositoryMemory {
    fn insert(&self, todo: Todo) -> Result<Todo> {
        self.todos.borrow_mut().push(todo.clone());
        return Ok(todo);
    }

    fn get_by_id(&self, id: String) -> Result<Todo> {
        let todos = self.todos.borrow();
        let todo = todos.iter()
            .filter(|todo| { id.eq(&todo.id) }).next();

        match todo {
            Some(item) => Ok(item.clone()),
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("Todo with {} not found in the database", id),
            ))
        }
    }
}