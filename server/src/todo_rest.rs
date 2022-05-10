use rusqlite::{Connection, Result};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncrState {
    pub name_hash: String,
    pub json: String
}

#[derive(Debug)]
pub struct Entry {
    pub name_hash: String,
    pub blob: String,
}

pub struct SQLWrapper {
    pub conn: Connection
}

impl SQLWrapper {
    pub fn get_encrstate(&self, name_hash: String) -> String {
        //TODO: implement it lol
"{\"dc7b5396c805c06f0ac60390322e4142a0bceb35\": {\"iv\": [-103, 37, -19, -3, 47, -121, -31, 79, 1, 54, 94, 56, -107, 100, -87, 104],\"salt\" : [1, 3, 3, 7, 4, 2, 0, 6, 9],\"algo\": \"AES/CBC/PKCS5Padding\"}}".to_string()
    }

    pub fn get_entry(name_hash: String) -> Result<Entry> {
        //TODO: same here, implement it!
        Ok(Entry {
            name_hash: "dc7b5396c805c06f0ac60390322e4142a0bceb35".to_owned(),
            blob: "9KGKWgHfgLKQ0TnRFvxSJcqZp+xn+l3miVcRsRgHQf0=n".to_owned(),
        })
    }
}

pub fn est_database_conn() -> Connection {
    //TODO: this has to be a file, will be implemented later
    let conn = Connection::open_in_memory().expect("failed to establish connection..\n aborting..");
    conn.execute("CREATE TABLE IF NOT EXISTS encrstate (name_hash TEXT PRIMARY KEY, json TEXT);", []);
    conn.execute("CREATE TABLE IF NOT EXISTS entry (name_hash TEXT PRIMARY KEY, blob TEXT);", []);
    conn
}