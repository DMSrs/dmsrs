use models::document::Document;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use postgres::rows::Row;
use models::correspondent::Correspondent;
use models::picture::Picture;
use chrono::{Date, DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use models::tag::Tag;

pub fn fetch_documents(pool : &Pool<PostgresConnectionManager>) -> Vec<Document> {
    let conn = pool.clone().get().unwrap();

    let mut documents: Vec<Document> = Vec::new();
    let query = conn.query("SELECT \
        documents.id,
        title, \
        correspondents.id as from_id, \
        correspondents.name as from_name, \
        \"date\", \
        pages \
     FROM documents \
     INNER JOIN correspondents ON correspondents.id = documents.correspondent \
     WHERE documents.hidden = false", &[]);
    if let Ok(rows) = query {
        for row in rows.iter() {
            //let src : String = row.get(0);
            documents.push(
                Document {
                    id: row.get(0),
                    title: row.get(1),
                    from: Correspondent {
                        id: row.get(2),
                        name: row.get(3),
                    },
                    date: Utc.from_utc_date(&(row.get::<usize, NaiveDate>(4)))
                        .and_hms(0,0,0),
                    image: Picture {
                        src: format!("documents/thumbnail/{}", row.get::<usize, i32>(0))
                    },
                    tags: Vec::new(),
                }
            );
        }

        for doc in documents.iter_mut() {
            doc.tags = fetch_tags_by_document(pool, &doc);
            println!("{:?}", doc.tags);
        }
    } else {
        let err = query.unwrap_err();
        println!("Unable to fetch rows! {}", err);
    }

    documents
}

pub fn fetch_tags_by_document(pool : &Pool<PostgresConnectionManager>, doc: &Document) -> Vec<Tag> {
    let conn = pool.clone().get().expect("Unable to get Pool Instance");
    let query = conn.query(r#"SELECT
        tags.slug,
        tags.name,
        tags.color
        FROM tags_documents
        INNER JOIN tags ON tags_documents.tag_slug = tags.slug
        WHERE tags_documents.document_id = $1
    "#, &[&doc.id]);

    let mut tags : Vec<Tag> = Vec::new();

    if let Ok(rows) = query {
        for row in rows.iter() {
            tags.push(Tag{
                slug: row.get(0),
                name: row.get(1),
                color: row.get(2)
            })
        }
    } else {
        println!("Unable to get tags: {}", query.unwrap_err());
    }

    tags
}