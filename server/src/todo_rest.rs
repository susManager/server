use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct EncrState {
    name_hash: String,
    iv: [isize; 16],
    salt: [isize; 9],
}

#[derive(Debug)]
struct Entry {
    name_hash: String,
    blob: Vec<u8>,
}

struct SQLWrapper {
    conn: Connection
}

impl SQLWrapper {
    pub fn get_encr_state(name_hash: String) -> Result<EncrState> {
        //TODO: implement it lol
        Ok(EncrState {
            name_hash: "fridolin".to_owned(),
            iv: [-103, 37, -19, -3, 47, -121, -31, 79, 1, 54, 94, 56, -107, 100, -87, 104],
           salt: [1, 3, 3, 7, 4, 2, 0, 6, 9],
        })
    }

    pub fn get_entry(name_hash: String) -> Result<Entry> {
        //TODO: same here, implement it!
        Ok(Entry {
            name_hash: "fridolin".to_owned(),
            blob: vec![1, 2, 3],
        })
    }
}