use std::io::{Error, Result};

use crate::domain::Todo;

pub trait TodoRepository {
    fn insert(&self, todo: Todo) -> Result<Todo>;
    fn get_by_id(&self, id: String) -> Result<Todo>;
    fn update(&self, todo: Todo) -> Result<Todo>;
    fn get_all(&self) -> Result<Vec<Todo>>;
    fn delete_by_id(&self, id: String) -> Option<Error>;
}
