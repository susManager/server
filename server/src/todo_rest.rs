use rusqlite::{Connection, Result};
use rocket::serde::{Serialize, Deserialize};
use uwuifier::uwuify_str_sse;

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
    pub fn get_encrstate(&self, name_hash: String) -> Result<String> {
        //TODO: implement it lol
 //"{\"dc7b5396c805c06f0ac60390322e4142a0bceb35\": {\"iv\": [-103, 37, -19, -3, 47, -121, -31, 79, 1, 54, 94, 56, -107, 100, -87, 104],\"salt\" : [1, 3, 3, 7, 4, 2, 0, 6, 9],\"algo\": \"AES/CBC/PKCS5Padding\"}}".to_string()
        let mut stmt = self.conn.prepare(format!("SELECT  json from encrstate where name_hash == \"{}\"", name_hash).as_str())?;
        let mut result = "".to_string();
        let iter = stmt.query_map([], |row| {
            Ok(EncrState {
                name_hash: name_hash.to_string(),
                json: row.get(0)?
            })
        })?;
        for r in iter {
            result.push_str(r.unwrap().json.as_str());
        }
        Ok(result)
    }

    pub fn get_data(&self, name_hash: String) -> Result<String> {
        let mut stmt = self.conn.prepare(format!("SELECT  blob from data where name_hash == \"{}\"", name_hash).as_str())?;
        let mut result = "".to_string();
        let iter = stmt.query_map([], |row| {
            Ok(EncrState {
                name_hash: name_hash.to_string(),
                json: row.get(0).unwrap()
            })
        }).unwrap();
        for r in iter {
            result.push_str(r.unwrap().json.as_str());
        }
        Ok(result)
    }

    pub fn insert_encrstate(&self, name_hash: String, json: String) -> String {
        let stmt;
        if self.get_encrstate(name_hash.clone()).unwrap().is_empty() {
            stmt = self.conn.execute("INSERT into encrstate (name_hash, json) values (?1, ?2);",
                [name_hash, json]);
        } else {
            uwuify_str_sse("for this user is already an encrstate saved, please use another username")
        };
        if stmt.is_ok() { "ok".to_string() } else { uwuify_str_sse(stmt.err().unwrap().to_string().as_str()) }.to_string()
    }

    pub fn insert_data(&self, name_hash: String, blob: String) -> String {
        let stmt;
        if self.get_data(name_hash.clone()).unwrap().is_empty() {
            stmt = self.conn.execute("INSERT into data (name_hash, blob) values (?1, ?2);",
                                     [name_hash, blob]);
        } else {
            stmt = self.conn.execute("UPDATE data set blob = ?2 where name_hash=?1;",
                                     [name_hash, blob]);
        };
        if stmt.is_ok() { "ok".to_string() } else { uwuify_str_sse(stmt.err().unwrap().to_string().as_str()) }.to_string()
    }
}

pub fn est_database_conn() -> Result<Connection> {
    //TODO: this has to be a file, will be implemented later
    let conn = Connection::open("./test-db.db3")?;
    conn.execute("CREATE TABLE IF NOT EXISTS encrstate (name_hash TEXT PRIMARY KEY, json TEXT);", [])?;
    //conn.execute("INSERT into encrstate (name_hash, json) values (\"fridolin\", \"{allah}\");", [])?;
    conn.execute("CREATE TABLE IF NOT EXISTS data (name_hash TEXT PRIMARY KEY, blob TEXT);", [])?;
    //conn.execute("INSERT into data (name_hash, blob) values (\"fridolin\", \"KGKWgHfgLKQ0TnRFvxSJcqZp+xn+l3miVcRsRgHQf0=\");", [])?;
    conn.flush_prepared_statement_cache();
    Ok(conn)
}