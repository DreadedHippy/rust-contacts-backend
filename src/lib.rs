use rusqlite::{Connection, Result};
pub mod routes;
pub mod models;
pub mod utils;

pub fn create_database() -> Result<Connection> {
    let database_connection = Connection::open("contacts.db")?;
    database_connection.execute("
        CREATE TABLE IF NOT EXISTS contacts  (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            number VARCHAR(15) NOT NULL UNIQUE,
            img_src TEXT,
            email TEXT
        );
    ", ())?;
	Ok(database_connection)
}