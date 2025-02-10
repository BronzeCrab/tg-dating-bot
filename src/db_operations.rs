use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::{Connection, Error, Result, Statement};

pub fn try_to_create_db_tables(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE user (
            id INTEGER PRIMARY KEY,
            tg_username TEXT NOT NULL UNIQUE,
            gender INTEGER,
            description TEXT NOT NULL
        );
        ",
        (),
    )?;
    conn.execute(
        "CREATE TABLE token (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE COLLATE NOCASE
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
) -> Result<u32, Error> {
    let mut stmt = conn
        .prepare(&format!(
            "INSERT INTO user (tg_username, description, gender) VALUES
            ('{tg_username}', '{description}', 1) RETURNING user.id;"
        ))
        .unwrap();
    let rows = stmt.query([]).unwrap();
    match rows.map(|r| r.get(0)).collect::<Vec<u32>>() {
        Ok(res) => return Ok(res[0]),
        Err(err) => return Err(err),
    };
}

fn get_token_id_by_name(conn: &Connection, token_name: String) -> Result<u32, Error> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT token.id FROM token WHERE token.name = '{token_name}';"
        ))
        .unwrap();

    let rows = stmt.query([]).unwrap();
    match rows.map(|r| r.get(0)).collect::<Vec<u32>>() {
        Ok(res) => Ok(res[0]),
        Err(err) => {
            return Err(err);
        }
    }
}

pub fn try_to_insert_user_tokens(
    conn: &Connection,
    tokens: Vec<String>,
) -> Result<Vec<u32>, Error> {
    let mut tokens_ids: Vec<u32> = Vec::new();
    for token in tokens {
        let mut stmt = conn
            .prepare(&format!(
                "INSERT INTO token (name) VALUES 
                ('{token}') RETURNING token.id;"
            ))
            .unwrap();

        let rows = stmt.query([]).unwrap();
        match rows.map(|r| r.get(0)).collect::<Vec<u32>>() {
            Ok(res) => {
                tokens_ids.push(res[0]);
            }
            Err(err) => {
                println!("we are here, and err is {err}");
                let token_id: u32 = get_token_id_by_name(&conn, token).unwrap();
                tokens_ids.push(token_id);
            }
        };
    }
    Ok(tokens_ids)
}

pub fn try_to_insert_user_token_relations(
    conn: &Connection,
    user_id: u32,
    tokens_ids: Vec<u32>,
) -> Result<(), Error> {
    for token_id in tokens_ids {
        let _ = conn.execute(
            &format!(
                "INSERT INTO user_token (user_id, token_id) VALUES 
                ({user_id}, {token_id});"
            ),
            (),
        );
    };
    Ok(())
}
