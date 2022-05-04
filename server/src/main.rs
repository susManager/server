mod todo_rest;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello from root"
}

#[get("/hi")]
fn hi() -> &'static str {
    "Hello there"
}

#[get("/greet/<name>")]
fn greeter(name: String) -> String {
    format!("Hello, {}!", name)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hi, greeter])
        .launch()
        .await;
}