use chrono::{NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};

use crate::schema::todos;
use crate::schema::todos::dsl::todos as all_todos;

#[table_name = "todos"]
#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
}

impl Todo {
    pub fn is_complete(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn is_incomplete(&self) -> bool {
        !self.is_complete()
    }

    /* DB Operations */

    pub fn all(conn: &PgConnection) -> Vec<Todo> {
        all_todos
            .order(todos::id.desc())
            .load::<Todo>(conn)
            .unwrap()
    }

    // TODO: return new record
    pub fn insert(todo: NewTodo, conn: &PgConnection) -> QueryResult<Todo> {
        diesel::insert_into(todos::table)
            .values(&todo)
            .get_result(conn)
    }

    // pub fn toggle_with_id(id: i32, conn: &PgConnection) -> bool {
    //     let todo = all_todos.find(id).get_result::<Todo>(conn);
    //     if todo.is_err() {
    //         return false;
    //     }

    //     let new_status = !todo.unwrap().is_complete();
    //     let updated_todo = diesel::update(all_todos.find(id));
    //     updated_todo
    //         .set(todo_completed.eq(new_status))
    //         .execute(conn)
    //         .is_ok()
    // }

    pub fn delete_with_id(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(all_todos.find(id)).execute(conn).is_ok()
    }
}

#[table_name = "todos"]
#[derive(Insertable, Debug)]
pub struct NewTodo {
    pub title: String,
    pub created_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
}

impl NewTodo {
    pub fn with_title<S: Into<String>>(title: S) -> Self {
        let created_at = Utc::now().naive_utc();

        NewTodo {
            title: title.into(),
            created_at,
            completed_at: None,
        }
    }
}
