use chrono;
use super::schema::*;

#[derive(Debug,Default,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug,Default,Serialize,Deserialize,Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations)]
#[belongs_to(Post)]
pub struct PostsRevision {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub post_id: i32,
    pub title: String,
    pub body: String,
}

impl Default for PostsRevision {
    fn default() -> Self {
        PostsRevision {
            id: i32::default(),
            created_at: chrono::Utc::now().naive_utc(),
            post_id: i32::default(),
            title: String::default(),
            body: String::default(),
        }
    }
}

#[derive(Debug,Default,Serialize,Deserialize,Insertable)]
#[table_name="posts_revisions"]
pub struct NewPostRevision {
    pub post_id: i32,
    pub title: String,
    pub body: String,
}

