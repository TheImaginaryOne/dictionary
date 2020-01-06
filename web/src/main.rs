use actix_web::{App, HttpServer};
use database::ConnectionPool;
use database::search::{DictSearch, load_search};
use std::sync::Arc;

use crate::controller::{pronunciation_search, character_search};

mod error;
mod controller;

struct AppData {
    database_pool: ConnectionPool,
    dict_search: Arc<DictSearch>,
}

fn main() {
    // the below line calls dotenv().ok()
    let database_pool = database::create_db_pool();
    let mut dict_search = DictSearch::new();
    println!("indexing data");
    load_search(&mut dict_search, &database_pool.get_connection());
    let ptr = Arc::new(dict_search);

    let address = std::env::var("BACKEND_ADDRESS").expect("BACKEND_ADDRESS must be defined");

    println!("Web service started at {}", address);
    HttpServer::new(move || {
        // AppData is data that is shared throughout the application.
        App::new().data(AppData { database_pool: database_pool.clone(), dict_search: ptr.clone() })
            .service(pronunciation_search)
            .service(character_search)
    }).bind(address).unwrap().run().unwrap();

    println!("Web service stopped");
}
