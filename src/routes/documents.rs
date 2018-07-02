use rocket::State;
use routes::RoutesHandler;
use std::fs::File;

use handlers::documenthandler::fetch_documents;
use rocket_contrib::Template;
use tera::Context;
use models::document::Document;
use handlers::documenthandler::fetch_document;

#[get("/documents/<id>")]
pub fn document_single(rh: State<RoutesHandler>, id: i32) -> Template {

    let mut context = Context::new();
    let document = fetch_document(&rh.pool, id);

    match document {
        Some(d) => {
            context.add("document", &d);
            Template::render("document", &context)
        },
        None => {
            Template::render("404", &context)
        }
    }
}

#[get("/documents")]
pub fn index(rh: State<RoutesHandler>) -> Template {

    let mut context = Context::new();
    let mut documents : Vec<Document> = Vec::new();

    let documents = fetch_documents(&rh.pool);

    context.add("documents", &documents);

    Template::render("documents", &context)
}

#[get("/documents/thumbnail/<id>")]
pub fn document_picture(rh: State<RoutesHandler>, id: i32) -> File {
    File::open("static/img/demo/document-1.jpg").unwrap()
}