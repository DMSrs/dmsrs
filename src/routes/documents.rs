use rocket::State;
use routes::RoutesHandler;
use std::fs::File;

use handlers::documenthandler::fetch_documents;
use models::document::Document;
use handlers::documenthandler::fetch_document;

use askama::Template;
use fairings::pathfairing::RocketPath;
use std::sync::Arc;
use handlers::documenthandler::get_document_thumbnail;
use std::path::Path;
use rocket::response::Responder;
use rocket::Response;
use rocket::http::Status;
use rocket::Request;
use rocket::http::ContentType;
use rocket::http::Header;
use handlers::documenthandler::get_document_filename;

#[derive(Template)]
#[template(path = "documents/single.html")]
pub struct SingleDocument<'a> {
    document: Document,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[derive(Template)]
#[template(path = "documents/index.html")]
pub struct MultipleDocuments<'a>{
    documents: Vec<Document>,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

pub struct ServeDocument {
    document_id: i32,
    document_name: String
}

impl Responder<'static> for ServeDocument {
    fn respond_to(self, _request: &Request) -> Result<Response<'static>, Status> {
        let path = format!("data/pdf/{}.pdf", self.document_id);
        if Path::new(&path).exists() {
            let cd = format!("attachment; filename=\"{}\"", self.document_name);

            return Response::build()
                .header(ContentType::new("application", "octet-stream"))
                .header(Header::new("Content-Description", "File Transfer"))
                .header(Header::new("Content-Disposition", cd))
                .sized_body(File::open(&path).unwrap())
                .ok();
        }

        return Err(Status::NotFound)
    }
}

#[get("/documents/view/<id>")]
pub fn document_view_single<'a>(_rh: State<'a, RoutesHandler>, id: i32) -> Option<File> {
    let path = format!("data/pdf/{}.pdf", id);
    if Path::new(&path).exists() {
        return Some(File::open(path).unwrap());
    }
    None
}

#[get("/documents/download/<id>")]
pub fn document_download_single<'a>(rh: State<'a, RoutesHandler>, id: i32) -> ServeDocument {
    ServeDocument {
        document_id: id,
        document_name: get_document_filename(&rh.pool, id)
    }
}

#[get("/documents/<id>")]
pub fn document_single<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>, id: i32) -> Option<SingleDocument<'a>> {
    let current_path : String = (*(path.path.lock().unwrap())).clone();
    let document = fetch_document(&rh.pool, id);

    document.map(|doc| SingleDocument { document: doc, rh, current_path })
}

#[get("/documents")]
pub fn index<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>) -> MultipleDocuments<'a> {
    let current_path : String = (*(path.path.lock().unwrap())).clone();
    let documents = fetch_documents(&rh.pool);
    MultipleDocuments { documents, rh, current_path }
}

#[get("/documents/thumbnail/<id>")]
pub fn document_picture(_rh: State<RoutesHandler>, id: i32) -> File {
    return get_document_thumbnail(id);
}

#[get("/documents/ocr/<id>")]
pub fn document_ocr(_rh: State<RoutesHandler>, id: i32) -> String {
    id.to_string()
}