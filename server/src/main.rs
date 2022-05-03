mod todo_rest;

use todo_rest::todos_filter;
use warp::Filter;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Enrc {
    iv: [u8; 16],
    salt: [u8; 9],
    algo: String

}

#[derive(Debug)]
struct Person {
    id: u32,
    name: String
}

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "hello from le root");

    let conn = Connection::open_in_memory().expect("lol22");
    conn.execute(
        "create table person (id integer primary key, name text not null)", []
    ).expect("lol");

    let hi = warp::path!("hi" / String)
        .map(|name| test(name, conn));

    let api = hi.or(todos_filter());

    let routes = api.or(root).or(hi);

    warp::serve(routes).run(([127,0,0,1], 5000)).await;

}

fn test(name: String, conn: Connection) -> String {
    conn.execute("insert into person (name) values (?1)", params![name.to_string()])
        .expect("entry failed");

    let mut stmt = conn.prepare("SELECT id, name FROM person where id < 3")
        .expect("select failed");

    let &mut test: String = "".to_owned();

    let person_iter = stmt.query_map([], |row| {
        Ok(Person{
            id: row.get(0)?,
            name: row.get(1)?
        })
    });

    for person in person_iter {
        test.push_str(" | ".to_owned().push_str(person.name?)?);
    }

    format!("Hello {}!", test)
}
