use std::io::Result;

use crate::domain::Todo;

pub trait TodoRepository {
    fn insert(&self, todo: Todo) -> Result<Todo>;
    fn get_by_id(&self, id: String) -> Result<Todo>;
}
