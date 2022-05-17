mod todo_rest;

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket::serde::{Deserialize, json::Json};
use todo_rest::SQLWrapper;
use std::sync::{Mutex};

struct TestState {
    db: Mutex<SQLWrapper>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Request {
    name_hash: String,
    content: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello from root"
}

#[post("/encrstate/<name_hash>", data = "<request>")]
fn post_encrstate(name_hash: String, request: Json<Request>, db: &State<TestState>) -> String {
    format!("{:?}", request.0.content)
}

#[get("/encrstate/<hash>")]
fn get_encrstate(hash: String, db: &State<TestState>) -> String {
    let res = db.db.lock().unwrap();
    res.get_encrstate(hash)
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
        .mount("/", routes![get_encrstate, get_data, post_encrstate])
        .launch()
        .await;
}