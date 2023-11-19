
use mysql::*;
use mysql::prelude::*;
use dotenv;

use crate::log_entry_struct::LogEntry;


pub async fn enter_log(log_data: LogEntry) -> Result<(), Box<dyn std::error::Error>>{

    // Connect to database

    // Load .env data
    dotenv::dotenv().ok();

    // Get Database Url from .env file
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Pass the database url as &str, instead of string.
    // Pool::new() is implemented only for &str
    let pool = Pool::new( &database_url[..] )?;

    let mut conn = pool.get_conn()?;

    // Create table if not exists
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS log_entries (
        id INT AUTO_INCREMENT PRIMARY KEY,
        level VARCHAR(255) NOT NULL,
        message TEXT NOT NULL,
        resourceId VARCHAR(255) NOT NULL,
        timestamp TIMESTAMP NOT NULL,
        traceId VARCHAR(255) NOT NULL,
        spanId VARCHAR(255) NOT NULL,
        commit VARCHAR(255) NOT NULL,
        parentResourceId VARCHAR(255) NOT NULL
        );")?;


    // Convert LogEntry to tuple
    let mut data = log_data.into_tuple();

    // Convert to valid Date Time
    data.3  = data.3.replace("Z", "");
    data.3  = data.3.replace("T", " ");

    // Build Query
    let query = format!("INSERT INTO log_entries (
            level,
            message,
            resourceId,
            timestamp,
            traceId,
            spanId,
            commit,
            parentResourceId
            ) VALUES ('{}','{}','{}','{}','{}','{}','{}','{}');
        ",
        data.0, data.1, data.2, data.3, data.4, data.5, data.6, data.7
    );

    println!("{}", query);

    conn.query_drop(query)?;

    println!("HELLO");
    Ok(())

}