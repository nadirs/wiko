extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate serde;

extern crate wiko;

use std::path;
use self::diesel::*;
use wiko::{Wiko,WikoConnection};
use wiko::models::*;


fn load_test_env() {
    let mut path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test.env");
    dotenv::from_path(&path).expect("Invalid dotenv");
}

pub struct TestHelper;

impl TestHelper {
    pub fn get_test_connection() -> WikoConnection {
        load_test_env();
        let database_url = dotenv::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL or dotenv missing");
        WikoConnection::establish(database_url.as_str()).expect("Error connecting to test DB")
    }

    pub fn new_wiko() -> Wiko {
        let conn = Self::get_test_connection();
        Wiko::new(conn)
    }

    pub fn insert_post() -> Post {
        let new_post = NewPost::default();
        Self::new_wiko().create_post(&new_post).expect("Error creating new post")
    }

    pub fn get_post() -> Post {
        let inserted = Self::insert_post();
        Self::new_wiko().get_post(inserted.id).expect("Error getting post")
    }

    pub fn insert_post_revision() -> PostRevision {
        let post = Self::get_post();
        let new_post_revision = NewPostRevision::from_post(&post);
        Self::new_wiko().create_post_revision(&new_post_revision).expect("Error creating new post revision")
    }
}
