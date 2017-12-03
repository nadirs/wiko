use chrono;
use super::schema::*;

#[derive(Debug,Default,Serialize,Deserialize,Queryable,Identifiable,Associations)]
#[table_name="posts"]
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
#[table_name="posts_revisions"]
pub struct PostRevision {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub post_id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Debug,Default,Serialize,Deserialize,Insertable)]
#[table_name="posts_revisions"]
pub struct NewPostRevision {
    pub post_id: i32,
    pub title: String,
    pub body: String,
}

impl NewPostRevision {
    pub fn from_post(post: &Post) -> Self {
        NewPostRevision {
            post_id: post.id,
            title: post.title.clone(),
            body: post.body.clone(),
        }
    }
}
