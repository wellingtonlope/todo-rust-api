use std::io::{Error, ErrorKind, Result};
use std::sync::Mutex;

use crate::app::repository::TodoRepository;
use crate::domain::Todo;

pub struct TodoRepositoryMemory {
    pub todos: Mutex<Vec<Todo>>,
}

impl TodoRepository for TodoRepositoryMemory {
    fn insert(&self, todo: Todo) -> Result<Todo> {
        let mut data = self.todos.lock().unwrap();
        data.push(todo.clone());
        Ok(todo)
    }

    fn get_by_id(&self, id: String) -> Result<Todo> {
        let data = self.todos.lock().unwrap();
        let todos = data;
        let todo = todos.iter()
            .filter(|todo| { id.eq(&todo.id) }).next();

        match todo {
            Some(item) => Ok(item.clone()),
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("Todo with id {} not found in the database", id),
            ))
        }
    }

    fn update(&self, todo: Todo) -> Result<Todo> {
        let mut data = self.todos.lock().unwrap();
        for update_todo in data.iter_mut() {
            if todo.id.eq(&update_todo.id) {
                *update_todo = todo.clone();
                return Ok(todo);
            }
        }

        Err(Error::new(
            ErrorKind::NotFound,
            format!("Todo with id {} not found in the database", todo.id),
        ))
    }

    fn get_all(&self) -> Result<Vec<Todo>> {
        Ok(self.todos.lock().unwrap().to_vec())
    }

    fn delete_by_id(&self, id: String) -> Option<Error> {
        let mut data = self.todos.lock().unwrap();
        let position = data.iter()
            .position(|item| {id.eq(&item.id)});
        if position.is_none() {
            return Some(Error::new(
                ErrorKind::NotFound,
                format!("Todo with id {} not found in the database", id),
            ));
        }
        let position = position.unwrap();

        data.remove(position);

        None
    }
}
