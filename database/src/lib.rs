#[macro_use]
pub extern crate diesel;

use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use dotenv::dotenv;
use std::env;
use diesel::connection::SimpleConnection;

pub mod schema;
pub mod models;

pub fn create_db_pool() -> ConnectionPool {
    dotenv().ok();
    let url = &env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let manager = ConnectionManager::<DbConnection>::new(url);
    ConnectionPool(Pool::builder().build(manager).expect(&format!("Could not create pool for database: {}", &url)))
}

pub type DbConnection = SqliteConnection;
pub struct ConnectionPool(Pool<ConnectionManager<DbConnection>>);
impl ConnectionPool {
    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<DbConnection>> {
        let conn = self.0.clone().get().unwrap();
        conn.batch_execute("pragma foreign_keys = on;").unwrap();
        conn
    }
}