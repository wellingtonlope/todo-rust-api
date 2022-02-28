pub struct GetAllTodo {
    repository: Rc<dyn TodoRepository>,
}

impl GetAllTodo {
    pub fn new(repository: Rc<dyn TodoRepository>) -> GetAllTodo {
        GetAllTodo { repository }
    }
}

#[derive(Debug, Clone)]
pub struct GetAllTodoInput {}

#[derive(Debug, Clone)]
pub struct GetAllTodoOutput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

impl GetAllTodoOutput {
    pub fn from_todo(todo: Todo) -> GetAllTodoOutput {
        GetAllTodoOutput {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            created_date: todo.created_date,
            updated_date: todo.updated_date,
        }
    }
}

impl UseCase<GetAllTodoInput, Vec<GetAllTodoOutput>> for GetAllTodo {
    fn handle(&self, _input: GetAllTodoInput) -> Result<Vec<GetAllTodoOutput>> {
        let todos = self.repository.get_all();
        if todos.is_err() {
            return Err(todos.unwrap_err());
        }
        let todos = todos.unwrap();

        Ok(todos.iter()
            .map(|item| { GetAllTodoOutput::from_todo(item.clone()) })
            .collect::<Vec<GetAllTodoOutput>>()
        )
    }
}