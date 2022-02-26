pub struct GetTodoById {
    repository: Rc<dyn TodoRepository>,
}

impl GetTodoById {
    pub fn new(repository: Rc<dyn TodoRepository>) -> GetTodoById {
        GetTodoById { repository }
    }
}

#[derive(Debug, Clone)]
pub struct GetTodoByIdInput {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct GetTodoByIdOutput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

impl GetTodoByIdOutput {
    pub fn from_todo(todo: Todo) -> GetTodoByIdOutput {
        GetTodoByIdOutput {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            created_date: todo.created_date,
            updated_date: todo.updated_date,
        }
    }
}

impl GetTodoById {
    pub fn handle(&self, input: GetTodoByIdInput) -> Result<GetTodoByIdOutput> {
        let todo = self.repository.get_by_id(input.id);
        if todo.is_err() {
            return Err(todo.unwrap_err());
        }

        Ok(GetTodoByIdOutput::from_todo(todo.unwrap()))
    }
}