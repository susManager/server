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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hi])
        .mount("/brian", routes![hi])
}

#[get("/greet/<seconds>")]
fn greeter() -> String {
    format!("Hello, {}!", seconds)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hi", routes![hi])
        .await;
}