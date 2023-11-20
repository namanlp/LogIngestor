#![allow(non_snake_case)]

// Declaring Modules
mod log_ingestor;
mod log_entry_struct;
mod query_interface;
mod database_operations;

use futures::join;

#[actix_web::main]
async fn main(){

    // Define Variable to run log ingestor
    let log_ingestor_task = log_ingestor::ingest();

    // Define Variable to run Query Interface
    let query_interface_task = query_interface::query_interface();

    // Run both Log Ingestor and Query interface tasks asynchronously
    join!(log_ingestor_task, query_interface_task);
}
