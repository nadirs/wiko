-- Your SQL goes here
CREATE TABLE posts_revisions (
    id SERIAL PRIMARY KEY,
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    post_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts (id)
)
