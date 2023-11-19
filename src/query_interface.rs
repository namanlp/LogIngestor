use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::database_operations::read_from_db;

// Function to take user input
async fn read_input() -> String {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);

    let mut input = String::new();
    let _ = reader.read_line(&mut input).await;
    input.trim().to_string()
}

pub async fn query_interface(){
    loop {

        println!("1. Level");
        println!("2. Message");
        println!("3. ResourceId");
        println!("4. TimeStamp");
        println!("5. TraceId");
        println!("6. SpanId");
        println!("7. Commit");
        println!("8. Metadata.parentResourceId");
        println!("9. Exit");

        println!("Choose the Option to Query Logs :");

        let choice = &read_input().await[..];

        // Run Queries According to the choice

        match choice {

            "1" => {
                println!("Enter Level : ");
                let level = read_input().await;
                let _ = read_from_db(format!("SELECT level, message, resourceId, timestamp,\
                 traceId, spanId, commit, parentResourceId from log_entries where level='{}'", level)).await;
            },
            "2" => {
                println!("Enter Message : ");
                let message = read_input().await;
                let _ = read_from_db(format!("SELECT level, message, resourceId, timestamp,\
                 traceId, spanId, commit, parentResourceId from log_entries where message like '%{}%'", message)).await;
            },
            "3" => {
                println!("Enter ResourceID : ");
                let resourceId = read_input().await;
                let _ = read_from_db(format!("SELECT level, message, resourceId, timestamp,\
                 traceId, spanId, commit, parentResourceId from log_entries where resourceId like'%{}%'", resourceId)).await;
            },
            "4" => {
                println!("Enter Start timestamp : ");
                let mut start_timestamp = read_input().await;
                println!("Enter End timestamp : ");
                let mut end_timestamp = read_input().await;

                start_timestamp = start_timestamp.replace("T", " ");
                start_timestamp = start_timestamp.replace("Z", "");

                end_timestamp = end_timestamp.replace("T", " ");
                end_timestamp = end_timestamp.replace("Z", "");

                let _ = read_from_db(format!("SELECT level, message, resourceId, timestamp,\
                 traceId, spanId, commit, parentResourceId from log_entries where timestamp BETWEEN'{}' AND '{}'", start_timestamp, end_timestamp)).await;
            },
            "5" => {
                println!("Enter traceId : ");
                let traceId = read_input().await;
                let _ = read_from_db(format!("SELECT level, message, resourceId, timestamp,\
                 traceId, spanId, commit, parentResourceId from log_entries where traceId='{}'", traceId)).await;
            },
            "6" => {
                println!("Enter spanId : ");
                let spanId = read_input().await;
                let _ = read_from_db(format!("SELECT level, message, resourceId, timestamp,\
                 traceId, spanId, commit, parentResourceId from log_entries where spanId='{}'", spanId)).await;
            },
            "7" => {
                println!("Enter commit : ");
                let commit = read_input().await;
                let _ = read_from_db(format!("SELECT level, message, resourceId, timestamp,\
                 traceId, spanId, commit, parentResourceId from log_entries where commit='{}'", commit)).await;
            },
            "8" => {
                println!("Enter parentResourceId : ");
                let parentResourceId = read_input().await;
                let _ = read_from_db(format!("SELECT level, message, resourceId, timestamp,\
                 traceId, spanId, commit, parentResourceId from log_entries where parentResourceId='{}'", parentResourceId)).await;
            },

            "9" => {
                println!("Thank You for your time!\nLog Ingestor will still be running!");
                break;
            }

            _ => println!("Sorry, wrong choice!\nTry Again")
        }

    }
}
