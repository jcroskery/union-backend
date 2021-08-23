use mysql_async::prelude::*;
use mysql_async::{params, Conn};
use std::error::Error;

const URL: &str = "mysql://justus:@localhost:3306/uniondb";

pub async fn create_tables() -> Result<(), Box<dyn Error>> {
    let pool = mysql_async::Pool::new(URL);
    pool.get_conn().await?.query_drop(r"CREATE TABLE IF NOT EXISTS users ( 
        id INT AUTO_INCREMENT PRIMARY KEY, 
        email VARCHAR(128) NOT NULL,
        password VARCHAR(255) NOT NULL
    );").await.expect("Failed to initialize user table.");
    pool.get_conn().await?.query_drop(r"CREATE TABLE IF NOT EXISTS galleries ( 
        id INT AUTO_INCREMENT PRIMARY KEY, 
        user INT NOT NULL,
        name VARCHAR(128) NOT NULL
    );").await.expect("Failed to initialize gallery table.");
    pool.get_conn().await?.query_drop(r"CREATE TABLE IF NOT EXISTS labels ( 
        id INT AUTO_INCREMENT PRIMARY KEY, 
        user INT NOT NULL,
        name VARCHAR(64) NOT NULL
    );").await.expect("Failed to initialize label table.");
    pool.get_conn().await?.query_drop(r"CREATE TABLE IF NOT EXISTS images ( 
        id INT AUTO_INCREMENT PRIMARY KEY, 
        user INT NOT NULL,
        name VARCHAR(128) NOT NULL
    );").await.expect("Failed to initialize label table.");
    pool.get_conn().await?.query_drop(r"CREATE TABLE IF NOT EXISTS labelmap ( 
        labelid INT NOT NULL, 
        imageid INT NOT NULL
    );").await.expect("Failed to initialize label map.");
    Ok(())
}
