use rocket::State;
use routes::RoutesHandler;
use std::fs::File;

#[get("/documents/thumbnail/<id>")]
pub fn document_picture(rh: State<RoutesHandler>, id: i32) -> File {
    File::open("static/img/demo/document-1.jpg").unwrap()
}