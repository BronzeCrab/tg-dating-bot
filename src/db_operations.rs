use rusqlite::{Connection, Error, Result, Statement};

pub fn try_to_create_db_tables(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE user (
            id    INTEGER PRIMARY KEY,
            description  TEXT NOT NULL,
            tg_username  TEXT NOT NULL
        );
        ",
        (),
    )?;
    Ok(())
}
