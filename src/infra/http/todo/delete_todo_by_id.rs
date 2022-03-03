pub async fn delete_todo_by_id(
    usecase: web::Data<DeleteTodoById>,
    id: web::Path<String>,
) -> HttpResponse {
    let output = usecase.handle(DeleteTodoByIdInput { id: id.into_inner() });
    if output.is_err() {
        return match output.unwrap_err().kind() {
            ErrorKind::NotFound => HttpResponse::NotFound().json(()),
            _ => HttpResponse::InternalServerError().json(())
        };
    }

    HttpResponse::NoContent().finish()
}
