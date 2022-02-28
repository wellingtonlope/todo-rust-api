use std::io::Result;
use std::rc::Rc;

use chrono::{DateTime, Utc};

use crate::{app::repository::TodoRepository, domain::Todo};

pub trait UseCase<T, R> {
    fn handle(&self, input: T) -> Result<R>;
}

include!("insert_todo.rs");
include!("get_todo_by_id.rs");
include!("update_todo.rs");
include!("get_all_todo.rs");
include!("delete_todo_by_id.rs");
