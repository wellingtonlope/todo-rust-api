#[derive(Clone)]
pub struct UpdateTodo {
    repository: Arc<dyn TodoRepository>,
}

impl UpdateTodo {
    pub fn new(repository: Arc<dyn TodoRepository>) -> UpdateTodo {
        UpdateTodo { repository }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateTodoInput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub updated_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct UpdateTodoOutput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

impl UpdateTodoOutput {
    pub fn from_todo(todo: Todo) -> UpdateTodoOutput {
        UpdateTodoOutput {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            created_date: todo.created_date,
            updated_date: todo.updated_date,
        }
    }
}

impl UseCase<UpdateTodoInput, UpdateTodoOutput> for UpdateTodo {
    fn handle(&self, input: UpdateTodoInput) -> Result<UpdateTodoOutput> {
        let update_todo = self.repository.get_by_id(input.id);
        if update_todo.is_err() {
            return Err(update_todo.unwrap_err());
        }
        let mut update_todo = update_todo.unwrap();

        let err = update_todo.update(input.title, input.description, input.updated_date);
        if err.is_some() {
            return Err(err.unwrap());
        }

        let updated_todo = self.repository.update(update_todo);
        if updated_todo.is_err() {
            return Err(updated_todo.unwrap_err());
        }

        Ok(UpdateTodoOutput::from_todo(updated_todo.unwrap()))
    }
}