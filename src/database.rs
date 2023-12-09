use rusqlite::{Connection, Result};

// opens a connection
pub fn open_connection() -> Result<Connection> {
    let connection = Connection::open("url_shortener.db")?; //  database name
    Ok(connection)
}

pub fn close_connection(connection: Connection) {
    connection.close().expect("Failed to close the connection");
}

// creates a table in the database to store short-long url as key-value pairs
pub fn create_table(connection: &Connection) -> Result<()> {
    // creates a table
    // urls is the file name within database
    // add UNIQUE to each
    const CREATE_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS urls (
        id VARCHAR PRIMARY KEY,
        url VARCHAR NOT NULL
    );
    "#;

    connection.execute(CREATE_TABLE_SQL, [])?;
    Ok(())
}

// inserts the short and long url into the 'urls' table
pub fn insert_url(connection: &Connection, short_url: &str, long_url: &str) -> Result<()> {
    let query = "INSERT INTO urls (id, url) VALUES (?, ?)";
    connection.execute(query, &[short_url, long_url])?;
    Ok(())
}

// returns the long url assiciated with the given short url
pub fn get_long_url(connection: &Connection, short_url: &str) -> Result<Option<String>> {
    let query = "SELECT url FROM urls WHERE id = ?";
    let mut stmt = connection.prepare(query)?;
    let result: Option<String> = stmt.query_row([short_url], |row| row.get(0))?;
    Ok(result)
}