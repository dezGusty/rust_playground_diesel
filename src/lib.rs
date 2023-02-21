pub mod models;
pub mod schema;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;

use self::models::{NewPost, Post, NewWeight};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))

    // .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(connection: &mut SqliteConnection, title: &'a str, body: &'a str) {
    use schema::posts;
    let new_post = NewPost {
        title,
        body,
        published: 0,
        entry_date: Some(chrono::Local::now().naive_local()),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(connection)
        .expect("Error saving new post");

    let results = posts::table
        .filter(posts::dsl::title.like(format!("%{}%", new_post.title)))
        .load::<Post>(connection)
        .expect("Error getting new post");

    for result in results {
        println!("{:?}", result);
    }
}

pub fn add_weight_entry(
    connection: &mut SqliteConnection,
    weight: f32,
    measurement_date: NaiveDateTime,
) {
    use schema::weights;
    let new_weight = NewWeight {
        weight,
        measurement_date,
    };

    diesel::insert_into(weights::table)
        .values(&new_weight)
        .execute(connection)
        .expect("Error inserting weight");
}
