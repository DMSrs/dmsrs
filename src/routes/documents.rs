use rocket::State;
use routes::RoutesHandler;
use std::fs::File;

use handlers::documenthandler::fetch_documents;
use models::document::Document;
use handlers::documenthandler::fetch_document;

use askama::Template;
use fairings::pathfairing::PathFairing;
use fairings::pathfairing::RocketPath;
use std::sync::Arc;
use handlers::documenthandler::get_document_thumbnail;

#[derive(Template)]
#[template(path = "document.html")]
pub struct SingleDocument<'a> {
    document: Document,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[derive(Template)]
#[template(path = "documents.html")]
pub struct MultipleDocuments<'a>{
    documents: Vec<Document>,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[get("/documents/<id>")]
pub fn document_single<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>, id: i32) -> Option<SingleDocument<'a>> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    let document = fetch_document(&rh.pool, id);

    document.map(|doc| SingleDocument { document: doc, rh, current_path })
}

#[get("/documents")]
pub fn index<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>) -> MultipleDocuments<'a> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    let mut documents : Vec<Document> = Vec::new();

    let documents = fetch_documents(&rh.pool);
    MultipleDocuments { documents, rh, current_path }
}

#[get("/documents/thumbnail/<id>")]
pub fn document_picture(rh: State<RoutesHandler>, id: i32) -> File {
    return get_document_thumbnail(id);
}