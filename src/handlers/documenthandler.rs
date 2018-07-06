use models::document::Document;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use postgres::rows::Row;
use models::correspondent::Correspondent;
use models::picture::Picture;
use chrono::{Date, DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use models::tag::Tag;
use std::fs::File;
use poppler::PopplerPage;
use poppler::PopplerDocument;
use cairo::prelude::*;
use cairo::ImageSurface;
use cairo::enums::Format::ARgb32;
use cairo::Context;
use std::path::Path;
use poppler::CairoSetSize;
use tesseract::Tesseract;

pub fn fetch_documents(pool : &Pool<PostgresConnectionManager>) -> Vec<Document> {
    let conn = pool.clone().get().unwrap();

    let mut documents: Vec<Document> = Vec::new();
    let query = conn.query("SELECT \
        documents.id,
        title, \
        correspondents.id as from_id, \
        correspondents.name as from_name, \
        \"date\", \
        added_on, \
        pages, \
        ocr_result \
     FROM documents \
     INNER JOIN correspondents ON correspondents.id = documents.correspondent \
     WHERE documents.hidden = false", &[]);
    if let Ok(rows) = query {
        for row in rows.iter() {
            //let src : String = row.get(0);
            documents.push(parse_document(&row));
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

pub fn fetch_documents_by_tag(pool : &Pool<PostgresConnectionManager>, slug: String) -> Vec<Document> {
    let conn = pool.clone().get().unwrap();

    let mut documents: Vec<Document> = Vec::new();
    let query = conn.query("SELECT \
        documents.id,
        title, \
        correspondents.id as from_id, \
        correspondents.name as from_name, \
        \"date\", \
        added_on, \
        pages, \
        ocr_result \
     FROM documents \
     INNER JOIN correspondents ON correspondents.id = documents.correspondent \
     WHERE documents.hidden = false \
     AND documents.id IN (SELECT document_id FROM tags_documents \
     WHERE tag_slug=$1)", &[&slug]);
    if let Ok(rows) = query {
        for row in rows.iter() {
            //let src : String = row.get(0);
            documents.push(parse_document(&row));
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
pub fn fetch_documents_by_correspondent(pool : &Pool<PostgresConnectionManager>, id: i32) -> Vec<Document> {
    let conn = pool.clone().get().unwrap();

    let mut documents: Vec<Document> = Vec::new();
    let query = conn.query("SELECT \
        documents.id,
        title, \
        correspondents.id as from_id, \
        correspondents.name as from_name, \
        \"date\", \
        added_on, \
        pages, \
        ocr_result \
     FROM documents \
     INNER JOIN correspondents ON correspondents.id = documents.correspondent \
     WHERE documents.hidden = false \
     AND documents.correspondent=$1", &[&id]);
    if let Ok(rows) = query {
        for row in rows.iter() {
            documents.push(parse_document(&row));
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

pub fn fetch_document(pool : &Pool<PostgresConnectionManager>, id: i32) -> Option<Document> {
    let conn = pool.clone().get().unwrap();

    let mut documents: Vec<Document> = Vec::new();
    let query = conn.query("SELECT \
        documents.id,
        title, \
        correspondents.id as from_id, \
        correspondents.name as from_name, \
        \"date\", \
        added_on, \
        pages, \
        ocr_result \
     FROM documents \
     INNER JOIN correspondents ON correspondents.id = documents.correspondent \
     WHERE documents.hidden = false AND documents.id=$1", &[&id]);
    if let Ok(rows) = query {
        if rows.is_empty() {
            return None
        }
        let row = rows.get(0);
        let mut document : Document = parse_document(&row);
        println!("{}", document.image.src);
        document.tags = fetch_tags_by_document(pool, &document);
        return Some(document);
    } else {
        println!("Unable to execute query: {}", query.unwrap_err());
    }

    None
}

pub fn parse_document(row: &Row) -> Document {
    return Document {
        id: row.get(0),
        title: row.get(1),
        from: Correspondent {
        id: row.get(2),
        name: row.get(3),
        },
        date: Utc.from_utc_date(&(row.get::<_, NaiveDate>(4)))
        .and_hms(0, 0, 0),
        added_on: Utc.from_utc_date(&(row.get::<_, NaiveDate>(5)))
            .and_hms(0, 0, 0),
        pages: row.get(6),
        ocr_result: row.get(7),
        image: Picture {
        src: format!("/documents/thumbnail/{}", row.get::<_, i32>(0))
        },
        tags: Vec::new()
    }
}

pub fn get_document_thumbnail(id: i32) -> File {
    let path = format!("data/pdf/{}.pdf", id);
    let thumbnail_path = format!("data/thumbnails/{}.png", id);

    if Path::new(&thumbnail_path).exists() {
        return File::open(&thumbnail_path).unwrap();
    }

    if ! Path::new(&path).exists() {
        return File::open("static/img/document-not-found.png").unwrap();
    }

    let doc : PopplerDocument = PopplerDocument::new_from_file(path, "").unwrap();
    let num_pages = doc.get_n_pages();
    let page : PopplerPage = doc.get_page(0).unwrap();
    let (w, h) = page.get_size();

    let mut surface = ImageSurface::create(ARgb32,  w as i32, h as i32).unwrap();
    let mut ctx = Context::new(&mut surface);

    ctx.save();
    page.render_for_printing(&mut ctx);
    ctx.restore();
    ctx.show_page();

    let mut f : File = File::create(&thumbnail_path).unwrap();
    surface.write_to_png(&mut f).expect("Unable to write PNG");
    File::open(&thumbnail_path).unwrap()
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

pub fn get_document_filename(pool : &Pool<PostgresConnectionManager>, id: i32) -> String {
    let document = fetch_document(pool, id);

    match document {
        Some(d) => {
            let clean_title = d.title.replace(" ", "_").replace('"', "").replace('/', "_");
            format!("{}_{}.pdf", d.date.format("%Y-%m-%d"), clean_title)
        }
        None => {
            String::from("unknown.pdf")
        }
    }
}

pub fn get_document_ocr(pool: &Pool<PostgresConnectionManager>, id: i32) -> String {
    let t :Tesseract = Tesseract::new() ;
    t.set_lang("ita");
    t.set_image("/tmp/ocr.jpg");
    t.recognize();
    return String::from(t.get_text());
}
