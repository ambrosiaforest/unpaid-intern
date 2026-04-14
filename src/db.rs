use rusqlite::{Row, params, Connection, Result, OptionalExtension};
use crate::Error;

#[derive(Debug)]
struct User {
    id: i32,
    user: String,
    balance: i32,
}

pub fn create_database() -> Result<(), Error> {
    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn = Connection::open("app_database.db")?;

    // Create a table named users
    conn.execute(
        "CREATE TABLE IF NOT EXISTS balances (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user TEXT NOT NULL,
                balance INTEGER NOT NULL
            )",
        [], // No parameters needed
    )?;

    println!("Database and table created successfully.");
    Ok(())
}

pub fn insert_user(user: &str, balance: i32) -> Result<(), Error> {
    let conn = Connection::open("app_database.db")?;

    // Insert a new user
    conn.execute(
        "INSERT INTO balances (user, balance) VALUES (?1, ?2)",
        params![user, balance], // Bind parameters
    )?;

    println!("User inserted successfully.");
    Ok(())
}

pub fn query_users() -> Result<(), Error> {
    let conn = Connection::open("app_database.db")?;

    // Retrieve data from users table
    let mut stmt = conn.prepare("SELECT id, user, balance FROM balances")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            user: row.get(1)?,
            balance: row.get(2)?,
        })
    })?;

    // Iterate over the retrieved rows
    for user in user_iter {
        println!("{:?}", user?);
    }

    Ok(())
}

pub fn get_balance(user: &str) -> Result<Option<i32>, Error> {
    let conn = Connection::open("app_database.db")?;

    let mut stmt = conn.prepare("SELECT balance FROM balances WHERE user = ?1 LIMIT 1")?;
    let balance_opt = stmt.query_row(params![user], |row: &Row| row.get::<_, i32>(0))
        .optional()?;

    Ok(balance_opt)

}

pub fn set_balance(user: &str, new_balance: i32) -> Result<(), Error> {
    let conn = Connection::open("app_database.db")?;

    conn.execute(
        "UPDATE balances SET balance = ?1 WHERE user = ?2",
        params![new_balance, user],
    )?;
    println!("Succesfully updated balance for {}", user);

    Ok(())
}
