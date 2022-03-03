#[derive(Clone)]
pub struct DeleteTodoById {
    repository: Arc<dyn TodoRepository>,
}

impl DeleteTodoById {
    pub fn new(repository: Arc<dyn TodoRepository>) -> DeleteTodoById {
        DeleteTodoById { repository }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteTodoByIdInput {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct DeleteTodoByIdOutput {}

impl UseCase<DeleteTodoByIdInput, DeleteTodoByIdOutput> for DeleteTodoById {
    fn handle(&self, input: DeleteTodoByIdInput) -> Result<DeleteTodoByIdOutput> {
        match self.repository.delete_by_id(input.id) {
            Some(err) => Err(err),
            None => Ok(DeleteTodoByIdOutput{})
        }
    }
}