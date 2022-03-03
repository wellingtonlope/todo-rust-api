#[derive(Debug, Deserialize, Serialize)]
pub struct GetTodoByIdResponse {
    id: String,
    title: String,
    description: String,
    created_date: String,
    updated_date: Option<String>,
}

impl GetTodoByIdResponse {
    pub fn from_get_todo_by_id_output(output: GetTodoByIdOutput) -> Self {
        GetTodoByIdResponse {
            id: output.id,
            title: output.title,
            description: output.description,
            created_date: DateTimeUtils::to_string(output.created_date),
            updated_date: if output.updated_date.is_some() {
                Some(DateTimeUtils::to_string(output.updated_date.unwrap()))
            } else {
                None
            },
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
        .json(GetTodoByIdResponse::from_get_todo_by_id_output(output.unwrap()))
}
