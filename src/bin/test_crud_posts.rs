extern crate wiko;
extern crate diesel;

use self::wiko::*;
use self::wiko::models::*;
use self::diesel::prelude::*;

fn main() {
    use wiko::schema::posts::dsl as posts;
    use wiko::schema::posts_revisions::dsl as posts_revisions;

    let connection = establish_connection();

    let new_post_revision = NewPostRevision {
        post_id: 19,
        title: "Hello world!".to_owned(),
        body: "Lorem ipsum, dolor sit amet.\nThe end".to_owned(),
    };

    let _updated: Post = diesel::update(posts::posts.filter(posts::id.eq(19)))
        .set(posts::title.eq("Good night moon"))
        .get_result(&connection)
        .expect("Error updating post");

    let inserted: PostsRevision = diesel::insert(&new_post_revision)
        .into(posts_revisions::posts_revisions)
        .get_result(&connection)
        .expect("Error creating new post revision");

    println!("{} {} {:?}", inserted.id, inserted.title, inserted.created_at);
    println!("{}", inserted.body);
    println!("----------\n");

    let posts = posts::posts.filter(posts::published.eq(true))
        .limit(5)
        .order(posts::id.asc())
        .load::<(Post)>(&connection)
        .expect("Error loading posts");

    let all_needed_revisions = PostsRevision::belonging_to(&posts)
        .order(posts_revisions::created.desc())
        .limit(5)
        .load::<PostsRevision>(&connection)
        .expect("Error loading revisions");

    let grouped_revisions = all_needed_revisions.grouped_by(&posts);
    let posts_with_revisions = posts.into_iter().zip(grouped_revisions).collect::<Vec<_>>();

    for (post, revisions) in posts_with_revisions {
        println!("{} {}", post.id, post.title);
        println!("{}", post.body);
        println!("{} revisions:", revisions.len());
        for revision in revisions {
            println!("| {} | {} | {}", revision.id, revision.created_at, revision.title);
        }
        println!("----------\n");
    }

    //let posts_to_delete = posts::posts.filter(posts::title.like("Hello world%"));
    //let num_deleted = diesel::delete(posts_to_delete)
    //    .execute(&connection)
    //    .expect(&format!("Error deleting post {}", inserted.id));
    //println!("Deleted {} posts", num_deleted);
}
