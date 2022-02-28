use std::io::ErrorKind;

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::app::usecase::{GetTodoById, GetTodoByIdInput, GetTodoByIdOutput, InsertTodo, InsertTodoInput, InsertTodoOutput, UseCase};
use crate::infra::http::DateTimeUtils;

const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoDTO {
    id: Option<String>,
    title: String,
    description: String,
    created_date: Option<String>,
    updated_date: Option<String>,
}

impl TodoDTO {
    fn from_get_todo_by_id_output(output: GetTodoByIdOutput) -> TodoDTO {
        TodoDTO {
            id: Some(output.id),
            title: output.title,
            description: output.description,
            created_date: Some(DateTimeUtils::to_string(output.created_date)),
            updated_date: if output.updated_date.is_some() {
                Some(DateTimeUtils::to_string(output.updated_date.unwrap()))
            } else {
                None
            },
        }
    }

    fn to_insert_todo_input(dto: TodoDTO) -> InsertTodoInput {
        InsertTodoInput {
            title: dto.title,
            description: dto.description,
            created_date: None
        }
    }

    fn from_insert_todo_output(output: InsertTodoOutput) -> TodoDTO {
        TodoDTO {
            id:  Some(output.id),
            title: output.title,
            description: output.description,
            created_date: Some(DateTimeUtils::to_string(output.created_date)),
            updated_date: None
        }
    }
}

pub async fn get_todo_by_id(
    usecase: web::Data<GetTodoById>,
    id: web::Path<String>,
) -> HttpResponse {
    let output = usecase.handle(GetTodoByIdInput { id: id.into_inner() });
    if output.is_err() {
        return match output.unwrap_err().kind() {
            ErrorKind::NotFound => HttpResponse::NotFound().json(()),
            _ => HttpResponse::InternalServerError().json(())
        };
    }

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(TodoDTO::from_get_todo_by_id_output(output.unwrap()))
}

pub async fn insert_todo(
    usecase: web::Data<InsertTodo>,
    item: web::Json<TodoDTO>
) -> HttpResponse {
    let output = usecase.handle(TodoDTO::to_insert_todo_input(item.into_inner()));
    if output.is_err() {
        return match output.unwrap_err().kind() {
            ErrorKind::InvalidInput => HttpResponse::BadRequest().json(()),
            _ => HttpResponse::InternalServerError().json(())
        }
    }

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(TodoDTO::from_insert_todo_output(output.unwrap()))
}