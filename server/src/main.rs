mod todo_rest;

#[macro_use]
extern crate rocket;

use rocket::State;
use todo_rest::SQLWrapper;
use std::sync::{Mutex};

struct TestState {
    db: Mutex<SQLWrapper>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello from root"
}

#[get("/encrstate/<hash>")]
fn get_encrstate(hash: String, db: &State<TestState>) -> String {
    let res = db.db.lock().unwrap();
    println!("{}", res.get_encrstate(hash.to_string()));
    serde_json::to_string(&res.get_encrstate(hash)).unwrap() //TODO: fix escape characters being serialized
}

#[get("/state/<hash>")]
fn get_data (hash: String) -> String {
    hash
}

#[rocket::main]
async fn main() {
    let db = TestState {db: Mutex::new(SQLWrapper {
        conn: todo_rest::est_database_conn()
    })};

    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount("/", routes![get_encrstate, get_data])
        .launch()
        .await;
}