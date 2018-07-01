#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate tera;

use chrono::prelude::*;
use rocket_contrib::Template;
use tera::Context;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::path::Path;

mod models;
use models::document::Document;
use models::tag::Tag;
use models::picture::Picture;

#[get("/")]
fn index() -> Template {

    let mut context = Context::new();
    let mut documents : Vec<Document> = Vec::new();
    documents.push(Document {
        title: String::from("Credit Card Statement"),
        from: "Postfinance".to_string(),
        date: Utc::now(),
        image: Picture {
            src: "img/demo/document-1.jpg".to_string(),
        },
        tags: vec![Tag{
            name: "Credit Card".to_string(),
            slug: "credit-card".to_string(),
            color: "#FFB74D".to_string()
        }],
    });

    documents.push(Document {
        title: String::from("Results"),
        date: Utc::now(),
        from: "Scuola Universitaria della Svizzera Italiana".to_string(),
        image: Picture {
            src: "img/demo/document-1.jpg".to_string(),
        },
        tags: vec![Tag{
            name: "School".to_string(),
            slug: "school".to_string(),
            color: "#00695C".to_string()
        }, Tag{
            name: "Personal".to_string(),
            slug: "personal".to_string(),
            color: "#9C27B0".to_string()
        }],
    });

    context.add("documents", &documents);

    Template::render("documents", &context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main(){
    rocket::ignite()
        .mount("/", routes![index, files])
        .attach(Template::fairing())
        .launch();
}
