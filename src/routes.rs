use crate::models::todo::{NewTodo, Todo};
use crate::DbConn;
use chrono::Utc;
use rocket::{http::Status, Route};
use rocket_contrib::json::Json;
use serde::Deserialize;

pub fn routes() -> Vec<Route> {
    routes![
        all_todos, add_todo,
        // mark_todo_as_complete,
        // mark_todo_as_incomplete
    ]
}

#[get("/")]
fn all_todos(conn: DbConn) -> Json<Vec<Todo>> {
    let todos = Todo::all(&conn);
    // let todos = vec![
    //     Todo::new_with_title("Take out the trash"),
    //     Todo::new_with_title("Wash the dishes"),
    //     Todo::new_with_title("Organize the pantry"),
    // ];

    Json(todos)
}

#[derive(Deserialize)]
struct NewTodoForm {
    title: String,
}

#[post("/", data = "<form>")]
fn add_todo(form: Json<NewTodoForm>, conn: DbConn) -> Result<Json<Todo>, Status> {
    let todo = NewTodo::with_title(form.into_inner().title);

    match Todo::insert(todo, &conn) {
        Ok(todo) => Ok(Json(todo)),
        _ => Err(Status::InternalServerError),
    }
}

// #[put("/<id>/complete")]
// fn mark_todo_as_complete(id: i32, conn: DbConn) -> Json<Todo> {
//     // TODO: fetch
//     let mut todo = Todo::with_title("Some TODO!");

//     if todo.is_incomplete() {
//         todo.completed_at = Some(Utc::now().naive_utc());
//     }

//     // TODO: handle if todo is not found
//     Json(todo)
// }

// #[put("/<id>/incomplete")]
// fn mark_todo_as_incomplete(id: i32, conn: DbConn) -> Json<Todo> {
//     // TODO: fetch
//     let mut todo = Todo::with_title("Some TODO!");
//     todo.completed_at = None;

//     // TODO: handle if todo is not found
//     Json(todo)
// }
