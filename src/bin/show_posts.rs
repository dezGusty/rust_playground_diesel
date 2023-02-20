extern crate weighter;
extern crate diesel;

use self::weighter::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
  use self::schema::posts::dsl::*;

  let mut connection = establish_connection();
  let results = posts
      .filter(published.eq(1))
      .limit(5)
      .load::<Post>(&mut connection)
      .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  for post in results {
      println!("{}", post.title);
      println!("-----------\n");
      println!("{}", post.body);
  }
}