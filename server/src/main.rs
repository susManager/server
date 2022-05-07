mod todo_rest;

#[macro_use]
extern crate rocket;

use rocket::State;
use rusqlite::{Connection};
use todo_rest::SQLWrapper;
use std::sync::{Arc, Mutex};

struct TestState {
    db: Mutex<SQLWrapper>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello from root"
}

#[get("/encrstate/<hash>")]
fn greeter(hash: String, db: &State<TestState>) -> String {
    let res = db.db.lock().unwrap();
    res.get_encr_state(hash).unwrap().name_hash
}

#[get("/state/<user>")]
fn get_state (user: String) -> String {
    user
}

#[rocket::main]
async fn main() {
    let db = TestState {db: Mutex::new(SQLWrapper {
        conn: Connection::open_in_memory().unwrap()
    })};

    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount("/", routes![greeter])
        .launch()
        .await;
}