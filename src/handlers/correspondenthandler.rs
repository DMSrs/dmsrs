use models::document::Document;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use postgres::rows::Row;
use models::correspondent::Correspondent;
use models::picture::Picture;
use chrono::{Date, DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use models::tag::Tag;

pub fn fetch_correspondents(pool : &Pool<PostgresConnectionManager>) -> Vec<Correspondent> {
    let conn = pool.clone().get().unwrap();

    let mut correspondents: Vec<Correspondent> = Vec::new();
    let query = conn.query("SELECT \
        id, name
        FROM correspondents", &[]);
    if let Ok(rows) = query {
        for row in rows.iter() {
            correspondents.push(parse_correspondent(&row));
        }
    } else {
        let err = query.unwrap_err();
        println!("Unable to fetch rows! {}", err);
    }

    correspondents
}

pub fn fetch_correspondent(pool : &Pool<PostgresConnectionManager>, id: i32) -> Option<Correspondent> {
    let conn = pool.clone().get().unwrap();

    let mut documents: Vec<Correspondent> = Vec::new();
    let query = conn.query("SELECT \
        id, name \
        FROM correspondents \
        WHERE correspondents.id=$1", &[&id]);
    if let Ok(rows) = query {
        if rows.is_empty() {
            return None
        }
        let row = rows.get(0);
        let mut correspondent : Correspondent = parse_correspondent(&row);
        return Some(correspondent);
    } else {
        println!("Unable to execute query: {}", query.unwrap_err());
    }

    None
}

pub fn parse_correspondent(row: &Row) -> Correspondent {
    return Correspondent {
        id: row.get(0),
        name: row.get(1)
    }
}