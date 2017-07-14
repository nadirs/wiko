use chrono;
use super::schema::*;

#[derive(Debug,Serialize,Deserialize,Queryable,Identifiable,Associations)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug,Serialize,Deserialize,Insertable)]
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

#[derive(Debug,Serialize,Deserialize,Insertable)]
#[table_name="posts_revisions"]
pub struct NewPostRevision {
    pub post_id: i32,
    pub title: String,
    pub body: String,
}

