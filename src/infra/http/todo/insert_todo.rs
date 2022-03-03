#[derive(Debug, Deserialize, Serialize)]
pub struct InsertTodoRequest {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertTodoResponse {
    id: String,
    title: String,
    description: String,
    created_date: String,
}

impl InsertTodoRequest {
    fn to_insert_todo_input(&self) -> InsertTodoInput {
        InsertTodoInput {
            title: self.title.clone(),
            description: self.description.clone(),
            created_date: None,
        }
    }
}

impl InsertTodoResponse {
    fn from_insert_todo_output(output: InsertTodoOutput) -> Self {
        InsertTodoResponse {
            id: output.id,
            title: output.title,
            description: output.description,
            created_date: DateTimeUtils::to_string(output.created_date),
        }
    }
}

pub async fn insert_todo(
    usecase: web::Data<InsertTodo>,
    body: web::Json<InsertTodoRequest>,
) -> HttpResponse {
    let output = usecase.handle(body.into_inner().to_insert_todo_input());
    if output.is_err() {
        return match output.unwrap_err().kind() {
            ErrorKind::InvalidInput => HttpResponse::BadRequest().json(()),
            _ => HttpResponse::InternalServerError().json(())
        };
    }

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(InsertTodoResponse::from_insert_todo_output(output.unwrap()))
}