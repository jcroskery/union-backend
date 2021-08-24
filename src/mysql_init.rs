use mysql::prelude::*;
use mysql::*;

pub fn get_conn() -> Conn {
    Conn::new(OptsBuilder::new().db_name(Some("uniondb")).user(Some("justus")).pass(Some(""))).expect("Failed to create pool")
}

pub fn create_tables() -> Result<()> {
    let mut conn = get_conn();
    conn.query_drop(r"CREATE TABLE IF NOT EXISTS users ( 
        id INT AUTO_INCREMENT PRIMARY KEY, 
        email VARCHAR(128) NOT NULL,
        username VARCHAR(64) NOT NULL,
        password VARCHAR(255) NOT NULL
    );").expect("Failed to initialize user table.");
    conn.query_drop(r"CREATE TABLE IF NOT EXISTS galleries ( 
        id INT AUTO_INCREMENT PRIMARY KEY, 
        user INT NOT NULL,
        name VARCHAR(128) NOT NULL
    );").expect("Failed to initialize gallery table.");
    conn.query_drop(r"CREATE TABLE IF NOT EXISTS labels ( 
        id INT AUTO_INCREMENT PRIMARY KEY, 
        user INT NOT NULL,
        name VARCHAR(64) NOT NULL
    );").expect("Failed to initialize label table.");
    conn.query_drop(r"CREATE TABLE IF NOT EXISTS images ( 
        id INT AUTO_INCREMENT PRIMARY KEY, 
        gallery INT NOT NULL,
        name VARCHAR(128) NOT NULL
    );").expect("Failed to initialize label table.");
    conn.query_drop(r"CREATE TABLE IF NOT EXISTS labelmap ( 
        labelid INT NOT NULL, 
        imageid INT NOT NULL
    );").expect("Failed to initialize label map.");
    conn.query_drop(r"CREATE TABLE IF NOT EXISTS activesessions ( 
        id VARCHAR(255) PRIMARY KEY, 
        user INT NOT NULL
    );").expect("Failed to initialize active session table.");
    Ok(())
}
