#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodoRequest {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodoResponse {
    id: String,
    title: String,
    description: String,
    created_date: String,
    updated_date: Option<String>,
}

impl UpdateTodoRequest {
    fn to_update_todo_input(&self, id: String) -> UpdateTodoInput {
        UpdateTodoInput {
            id,
            title: self.title.clone(),
            description: self.description.clone(),
            updated_date: None,
        }
    }
}

impl UpdateTodoResponse {
    fn from_update_todo_output(output: UpdateTodoOutput) -> Self {
        UpdateTodoResponse {
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

pub async fn update_todo(
    usecase: web::Data<UpdateTodo>,
    id: web::Path<String>,
    body: web::Json<UpdateTodoRequest>,
) -> HttpResponse {
    let output = usecase.handle(body.into_inner().to_update_todo_input(id.into_inner()));
    if output.is_err() {
        return match output.unwrap_err().kind() {
            ErrorKind::InvalidInput => HttpResponse::BadRequest().json(()),
            _ => HttpResponse::InternalServerError().json(())
        };
    }

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(UpdateTodoResponse::from_update_todo_output(output.unwrap()))
}