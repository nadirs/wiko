#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate dotenv;
extern crate serde;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
#[cfg(test)] use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
pub fn establish_test_connection() -> Result<SqliteConnection, diesel::ConnectionError> {
    SqliteConnection::establish("../test.sqlite")
}

#[cfg(test)]
pub mod test {
    use establish_test_connection;

    #[test]
    pub fn test_establish_connection() {
        establish_test_connection()
            .expect("Error connecting to test DB");
    }
}
