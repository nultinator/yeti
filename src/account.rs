use crate::{Result, DATA_PATH};
use rusqlite::{Connection, NO_PARAMS};

pub fn get_balance() -> Result<()> {
    let data_connection = Connection::open(DATA_PATH)?;
    let balance = data_connection.query_row(
        "SELECT SUM(value) FROM received_notes WHERE spent IS NULL",
        NO_PARAMS,
        |row| row.get(0).or(Ok(0)),
    )?;
    println!("Balance: {}", balance);

    Ok(())
}
