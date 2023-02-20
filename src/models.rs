use chrono::{NaiveDateTime};
use diesel::prelude::*;

use super::schema::posts;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: i32,
    pub entry_date: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: i32,
    pub entry_date: Option<NaiveDateTime>,
}
