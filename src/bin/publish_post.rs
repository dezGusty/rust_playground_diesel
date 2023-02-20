use self::models::Post;
use diesel::prelude::*;
use weighter::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(1))
        .execute(connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post");
    // println!("Published post {}", post.title);
}