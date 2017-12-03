#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_derives;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate serde;
pub mod schema;
pub mod models;

use std::env;
use std::iter::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use models::{Post,NewPost,PostRevision,NewPostRevision};
use schema::posts::dsl as posts_dsl;
use schema::posts_revisions::dsl as posts_revisions_dsl;

pub type WikoConnection = PgConnection;
type DieselResult<T> = Result<T, diesel::result::Error>;

pub fn establish_connection() -> WikoConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct Wiko {
    connection: WikoConnection,
}

impl Wiko {
    pub fn new(connection: WikoConnection) -> Self {
        Wiko {
            connection: connection
        }
    }

    pub fn get_published_posts(&self) -> Vec<Post> {
        posts_dsl::posts
            .filter(posts_dsl::published.eq(true))
            .order(posts_dsl::id.asc())
            .load::<(Post)>(&self.connection)
            .expect("Unable to load get_published_posts")
            .into_iter()
            .collect()
    }

    pub fn get_post(&self, id: i32) -> DieselResult<Post> {
        posts_dsl::posts
            .filter(posts_dsl::id.eq(id))
            .first(&self.connection)
    }

    pub fn create_post(&self, post: &NewPost) -> DieselResult<Post> {
        diesel::insert_into(posts_dsl::posts)
            .values(post)
            .get_result(&self.connection)
    }

    pub fn create_post_revision(&self, post_revision: &NewPostRevision) -> DieselResult<PostRevision> {
        diesel::insert_into(posts_revisions_dsl::posts_revisions)
            .values(post_revision)
            .get_result(&self.connection)
    }
}
