use rusqlite::{params, Connection, Result};

use crate::constants::defaults::DATABASE_NAME;

pub fn get(id: u32, column: &str, table: &str) -> Result<String, rusqlite::Error>{
    match Connection::open(DATABASE_NAME){
        Ok(connection) => {
            //connection.execute("INSERT INTO main DEFAULT VALUES;", [])?;
            let data = connection.query_row(
                format!("SELECT {} FROM {} WHERE id = ?1", column, table).as_str(),
                params![id],
                |row| row.get(0 as usize),
            );
            match data {
                Ok(d) => {
                    return Ok(d)
                },
                Err(error) => {
                    return Err(error);
                },
            }
        },
        Err(error) => {
            return Err(error);
        },
    };
}