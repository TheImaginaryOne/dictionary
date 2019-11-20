#[macro_use]
pub extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn create_db_pool() -> ConnectionPool {
    dotenv().ok();
    let url = &env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder().build(manager).expect(&format!("Could not create pool for database: {}", &url))
}

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PgConnection;