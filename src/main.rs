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
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

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

mod routes;
use routes::RoutesHandler;

mod handlers;

use models::correspondent::Correspondent;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;
use r2d2::Pool;

#[get("/css/<file..>")]
fn css_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/css/").join(file)).ok()
}

#[get("/img/<file..>")]
fn img_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/img/").join(file)).ok()
}

#[get("/fonts/<file..>")]
fn fonts_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/fonts/").join(file)).ok()
}

fn main(){

    let manager =
        PostgresConnectionManager::new(
            "postgres://postgres:default@172.17.0.2/postgres",
            TlsMode::None).expect("Unable to connect to DB");

    let pool = Pool::new(manager).expect("Unable to create Pool");

    let rh = RoutesHandler{ pool };

    rocket::ignite()
        .manage(rh)
        .mount("/", routes![
        routes::documents::index,
        routes::documents::document_single,
        routes::documents::document_picture,
        routes::tags::index,
        css_files,
        img_files,
        fonts_files])
        .attach(Template::fairing())
        .launch();
}
