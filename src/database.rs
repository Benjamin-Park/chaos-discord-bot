use rusqlite::*;

pub fn connect(database_file: String) -> Result<rusqlite::Connection> {
    let connection = Connection::open(database_file.to_string()).expect("error connecting to database");

    Ok(connection)
}


