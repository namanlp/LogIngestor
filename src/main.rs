#![allow(non_snake_case)]

// Declaring Modules
mod log_ingestor;
mod log_entry_struct;
mod query_interface;

// Using Functions
use futures::join;
use log_ingestor::ingest;

#[actix_web::main]
async fn main(){

    // Define Variable to run log ingestor
    let log_ingestor_thread = ingest();
    join!(log_ingestor_thread);
}
