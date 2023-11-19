use actix_web::{web, App, HttpServer, HttpResponse, Responder};

use crate::log_entry_struct::LogEntry;
use crate::database_operations::enter_log;


// This
pub async fn ingest() -> std::io::Result<()> {
    // Here, we are reading the log entry from port 3000
    // And pass it to Log Ingestor
    HttpServer::new(|| { App::new()
        // Take Logs from path /
            .service(web::resource("/")
                .route(web::post()
                    .to(log_ingestor)))
        // Bind logs to address localhost:3000
    }).bind("127.0.0.1:3000")?
        .run().await
}

async fn log_ingestor(data: web::Json<LogEntry>) -> impl Responder {

    // Handling Log Entry Data
    // Replace with Database Operation Later
    // println!("Received log entry: {:?}", data);
    enter_log(data.into_inner()).await.unwrap();

    // If log was ingested successfully, return success message
    HttpResponse::Ok().body("Log entry received successfully")
}

