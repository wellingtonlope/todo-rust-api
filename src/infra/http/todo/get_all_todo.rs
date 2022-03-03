use crate::app::usecase::{GetAllTodo, GetAllTodoInput};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAllTodoResponse {
    id: String,
    title: String,
    description: String,
    created_date: String,
    updated_date: Option<String>,
}

impl GetAllTodoResponse {
    pub fn from_get_all_todo_output(output: GetAllTodoOutput) -> Self {
        GetAllTodoResponse {
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

pub async fn get_all_todo(
    usecase: web::Data<GetAllTodo>,
) -> HttpResponse {
    let output = usecase.handle(GetAllTodoInput {});
    if output.is_err() {
        return HttpResponse::InternalServerError().json(());
    }

    let output = output.unwrap().iter()
        .map(|output| GetAllTodoResponse::from_get_all_todo_output(output.clone()))
        .collect::<Vec<GetAllTodoResponse>>();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(output)
}
