use std::io::{stdin, Read};
use chrono::{NaiveDateTime};
use weighter::*;
use std::env::args;

fn main() {
    let connection = &mut establish_connection();

    let in_weight: String = args().nth(1).expect("Expected argument %cmd% weight date");
    let in_date: String = args().nth(2).expect("Expected argument %cmd% weight date");

    let weight = in_weight.parse::<f32>().unwrap();
    let in_date: NaiveDateTime = in_date.parse::<NaiveDateTime>().unwrap();

    // println!("What would you like your title to be?");
    // stdin().read_line(&mut title).unwrap();
    // let title = title.trim_end(); // Remove the trailing newline

    // println!(
    //     "\nOk! Let's write {} (Press {} when finished)\n",
    //     title, EOF
    // );
    // stdin().read_to_string(&mut body).unwrap();

    add_weight_entry(connection, weight, in_date);
    // println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
