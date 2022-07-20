mod todo_rest;

#[macro_use]
extern crate rocket;

use std::collections::LinkedList;
use std::io::Bytes;
use rocket::State;
use rocket::serde::{Deserialize, json::Json};
use todo_rest::SQLWrapper;
use std::sync::{Mutex};

use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};

struct TestState {
    db: Mutex<SQLWrapper>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Request {
    content: String,
    sign: String
}

fn decode(data: &String) -> Vec<u8> {
    let mut splitted = data.split("<>");
    let mut vec : Vec<u8> =  Vec::new();
    splitted.fold(vec, |&acc, s|
        acc.push(s_to_byte(s.to_string()))
    )
}

fn s_to_byte(s: String) -> u8{
    let i: u8 = s.parse().unwrap();
    i
}

#[get("/")]
fn index() -> &'static str {
    "Hello from root! susManager-Server has started successfully!"
}

//https://rocket.rs/v0.5-rc/guide/requests/#body-data
#[post("/encrstate/<name_hash>", data = "<request>")]
fn post_encrstate(name_hash: String, request: Json<Request>, db: &State<TestState>) -> String {
    let res = db.db.lock().unwrap();
    res.insert_encrstate(name_hash, &request.content, &request.sign)
}

#[get("/exists/<name_hash>")]
fn exists(name_hash: String, db: &State<TestState>) -> &'static str {
    if db.db.lock().unwrap().exists(name_hash) {
        "yes"
    } else {
        "no"
    }
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
    let data = decode(&request.content);
    res.insert_data(name_hash, &request.content)
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
        .mount("/", routes![
            get_encrstate, get_data,
            post_encrstate, post_data,
            exists])
        .launch()
        .await
        .unwrap();
}