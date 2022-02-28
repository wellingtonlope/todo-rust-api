pub struct InsertTodo {
    repository: Rc<dyn TodoRepository>,
}

impl InsertTodo {
    pub fn new(repository: Rc<dyn TodoRepository>) -> InsertTodo {
        InsertTodo { repository }
    }
}

#[derive(Debug, Clone)]
pub struct InsertTodoInput {
    pub title: String,
    pub description: String,
    pub created_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct InsertTodoOutput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_date: DateTime<Utc>,
}

impl InsertTodoOutput {
    pub fn from_todo(todo: Todo) -> InsertTodoOutput {
        InsertTodoOutput {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            created_date: todo.created_date,
        }
    }
}

impl UseCase<InsertTodoInput, InsertTodoOutput> for InsertTodo {
    fn handle(&self, input: InsertTodoInput) -> Result<InsertTodoOutput> {
        let todo = Todo::new(input.title, input.description, input.created_date);
        if todo.is_err() {
            return Err(todo.unwrap_err());
        }

        let saved_todo = self.repository.insert(todo.unwrap());
        if saved_todo.is_err() {
            return Err(saved_todo.unwrap_err());
        }

        Ok(InsertTodoOutput::from_todo(saved_todo.unwrap()))
    }
}