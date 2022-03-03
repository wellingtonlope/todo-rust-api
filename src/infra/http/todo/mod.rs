use std::io::ErrorKind;

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::app::usecase::{DeleteTodoById, DeleteTodoByIdInput, UpdateTodoInput,UpdateTodoOutput,UpdateTodo, GetAllTodoOutput, GetTodoById, GetTodoByIdInput, GetTodoByIdOutput, InsertTodo, InsertTodoInput, InsertTodoOutput, UseCase};
use crate::infra::http::DateTimeUtils;

const APPLICATION_JSON: &str = "application/json";

include!("insert_todo.rs");
include!("get_todo_by_id.rs");
include!("get_all_todo.rs");
include!("update_todo.rs");
include!("delete_todo_by_id.rs");