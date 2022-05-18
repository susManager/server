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
    content: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello from root"
}

//https://rocket.rs/v0.5-rc/guide/requests/#body-data
#[post("/encrstate/<name_hash>", data = "<request>")]
fn post_encrstate(name_hash: String, request: Json<Request>, db: &State<TestState>) -> String {
    let res = db.db.lock().unwrap();
    res.insert_encrstate(name_hash, request.content.clone())
}

#[get("/encrstate/<name_hash>")]
fn get_encrstate(name_hash: String, db: &State<TestState>) -> String {
    let res = db.db.lock().unwrap();
    let a = res.get_encrstate(name_hash);
    match a {
        Ok(a) => if a.is_empty() {
            "nothing found".to_string()
        } else { a }
        Err(_a) => "something went wrong".to_string()
    }
}

#[post("/data/<name_hash>", data = "<request>")]
fn post_data(name_hash: String, request: Json<Request>, db: &State<TestState>) -> String {
    let res = db.db.lock().unwrap();
    res.insert_data(name_hash, request.content.clone())
}

#[get("/data/<name_hash>")]
fn get_data (name_hash: String, db: &State<TestState>) -> String {
    let res = db.db.lock().unwrap();
    let a = res.get_data(name_hash);
    match a {
        Ok(a) => if a.is_empty() {
            "nothing found".to_string()
        } else { a }
        Err(_a) => "something went wrong".to_string()
    }
}

#[rocket::main]
async fn main() {
    let db = TestState {db: Mutex::new(SQLWrapper {
        conn: todo_rest::est_database_conn().expect("failed to establish connection")
    })};

    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount("/", routes![get_encrstate, get_data,
            post_encrstate, post_data])
        .launch()
        .await.
        expect("failed to start uwu");
}