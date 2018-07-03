use rocket::State;
use routes::RoutesHandler;
use std::fs::File;

use handlers::documenthandler::fetch_documents;
use tera::Context;
use models::document::Document;
use handlers::documenthandler::fetch_document;

use askama::Template;

#[derive(Template)]
#[template(path = "document.html")]
pub struct SingleDocument<'a> {
    document: Document,
    rh: State<'a, RoutesHandler>
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct Error404{}

#[derive(Template)]
#[template(path = "documents.html")]
pub struct MultipleDocuments<'a>{
    documents: Vec<Document>,
    rh: State<'a, RoutesHandler>
}

#[get("/documents/<id>")]
pub fn document_single(rh: State<RoutesHandler>, id: i32) -> Option<SingleDocument> {

    let mut context = Context::new();
    let document = fetch_document(&rh.pool, id);

    document.map(|doc| SingleDocument { document: doc, rh})
}

#[get("/documents")]
pub fn index(rh: State<RoutesHandler>) -> MultipleDocuments {

    println!("Viewing: {}", *rh.current_page.lock().unwrap());

    let mut context = Context::new();
    let mut documents : Vec<Document> = Vec::new();

    let documents = fetch_documents(&rh.pool);
    MultipleDocuments { documents, rh }
}

#[get("/documents/thumbnail/<id>")]
pub fn document_picture(rh: State<RoutesHandler>, id: i32) -> File {
    File::open("static/img/demo/document-1.jpg").unwrap()
}