#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use dotenv::dotenv;

#[database("todos")]
pub struct DbConn(diesel::PgConnection);

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/todos", routes::routes())
}

fn main() {
    dotenv().ok();

    rocket().launch();
}
