use rusqlite::{Connection, Error, Result, Statement};

pub fn try_to_create_db_tables(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE user (
            id    INTEGER PRIMARY KEY,
            tg_username  TEXT NOT NULL UNIQUE,
            description  TEXT NOT NULL
        );
        ",
        (),
    )?;
    Ok(())
}

pub fn try_to_insert_user_data(
    conn: &Connection,
    tg_username: &str,
    description: &str,
) -> Result<(), Error> {
    let res = conn.execute(
        &format!(
            "INSERT INTO user (tg_username, description) VALUES
            ('{tg_username}', '{description}') RETURNING user.id;"
        ),
        (),
    );
    Ok(())
}
