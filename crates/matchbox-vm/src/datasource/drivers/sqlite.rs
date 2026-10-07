//This is the driver for the implementation of SQLite. 

use std::sync::Mutex;   //Keeps code consistant, acts like a lock.
use rusqlite::Connection;   //Connects SQLite to the rusqlite library.
use crate::datasource::traits::{DatasourceConfig, DbDriver, QueryParam, QueryResult};   //Takes matchbox's types to be converted to SQL

pub struct SqliteDriver {
    conn: Mutex<Connection>, //Lock keeps two queries from collision.
}

//Build the driver: opens the db named in the config and creates a new db in RAM.
impl SqliteDriver {
    pub fn new(config: &DatasourceConfig) -> Result<Self, String> {
        let conn = Connection::open(&config.database)
            .map_err(|e| format!("Failed to open SQLite database '{}': {}", config.database, e))?;  //Fail: return specific error to user, instead of exiting suddenly.
        Ok(SqliteDriver { conn: Mutex::new(conn) }) //Pass: return OK.
    }
}

//We have to make the SQLite driver fit into the rulebook of the DbDriver already set.

impl DbDriver for SqliteDriver {
    fn name(&self) -> &str {
        "sqlite"
    }
    //TO DO: Queries do not work yet. The _ in front of the conn tells rust that it's unused, so it automatically throws the error. 
    fn execute(&self, _sql: &str, _params: &[QueryParam]) -> Result<QueryResult, String> {
        let _conn = self.conn.lock().map_err(|_| "SQLite connection lock failed".to_string())?;
        Err("SQLite execute not implemented yet".to_string())
    }
}