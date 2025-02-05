use rusqlite::{Connection, Error, Result, Statement};

pub fn try_to_create_db_tables(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE user (
            id INTEGER PRIMARY KEY,
            tg_username TEXT NOT NULL UNIQUE,
            description TEXT NOT NULL
        );
        ",
        (),
    )?;
    conn.execute(
        "CREATE TABLE token (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            idf REAL NOT NULL
        );
        ",
        (),
    )?;
    conn.execute(
        "CREATE TABLE user_token (
            user_id INT NOT NULL,
            token_id INT NOT NULL,
            PRIMARY KEY (user_id, token_id),
            FOREIGN KEY (user_id) REFERENCES user,
            FOREIGN KEY (token_id) REFERENCES token
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
    let _res = conn.execute(
        &format!(
            "INSERT INTO user (tg_username, description) VALUES
            ('{tg_username}', '{description}') RETURNING user.id;"
        ),
        (),
    );
    Ok(())
}

pub fn try_to_insert_user_token_relations(
    conn: &Connection,
    user_id: u32,
    tokens_ids: Vec<u32>,
) -> Result<(), Error> {
    for token_id in tokens_ids {
        let _ = conn.execute(
            &format!(
                "INSERT INTO user_token (user_id, tokens_ids) VALUES 
                ({user_id}, {token_id});"
            ),
            (),
        );
    }
    Ok(())
}
