use models::document::Document;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use postgres::rows::Row;
use models::correspondent::Correspondent;
use models::picture::Picture;
use chrono::{Date, DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use models::tag::Tag;
use std::cmp::Ordering;

pub fn fetch_tags(pool : &Pool<PostgresConnectionManager>) -> Vec<Tag> {
    let conn = pool.clone().get().unwrap();

    let mut tags: Vec<Tag> = Vec::new();
    let query = conn.query("SELECT \
        slug, \
        name, \
        color
     FROM tags \
     WHERE hidden = false", &[]);
    if let Ok(rows) = query {
        for row in rows.iter() {
            //let src : String = row.get(0);
            tags.push(
                Tag {
                    slug: row.get(0),
                    name: row.get(1),
                    color: row.get(2)
                }
            );
        }
    } else {
        let err = query.unwrap_err();
        println!("Unable to fetch rows! {}", err);
    }

    tags.sort_by(|a: &Tag, b: &Tag|
        match a.name < b.name  {
            true => Ordering::Less,
            false => Ordering::Greater
        }
    );

    tags
}

pub fn fetch_tag(pool: &Pool<PostgresConnectionManager>, slug: String) -> Option<Tag> {
    let conn = pool.clone().get().unwrap();

    let query = conn.query("SELECT slug, name, color \
    FROM tags \
    WHERE tags.slug = $1", &[&slug]);

    if let Ok(rows) = query {
        if rows.is_empty() {
            return None;
        }

        let row = rows.get(0);
        return Some(Tag {
                slug: row.get(0),
                name: row.get(1),
                color: row.get(2)
            }
        );
    } else {
        let err = query.unwrap_err();
        println!("Unable to fetch rows! {}", err);
    }
    None
}