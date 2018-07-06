use rocket::State;
use routes::RoutesHandler;
use std::fs::File;

use askama::Template;
use fairings::pathfairing::PathFairing;
use fairings::pathfairing::RocketPath;
use std::sync::Arc;
use models::correspondent::Correspondent;
use handlers::correspondenthandler::fetch_correspondent;
use handlers::correspondenthandler::fetch_correspondents;
use handlers::documenthandler::fetch_documents_by_correspondent;
use models::document::Document;
use std::collections::HashMap;
use rocket::request::Form;
use rocket::response::Redirect;
use handlers::correspondenthandler::add_correspondent;
use handlers::correspondenthandler::remove_correspondent;

static MODULE_PATH : &str = "/correspondents";

#[derive(Template)]
#[template(path = "correspondents/single.html")]
pub struct SingleCorrespondent<'a> {
    correspondent: Correspondent,
    documents: Vec<Document>,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[derive(Template)]
#[template(path = "correspondents/add.html")]
pub struct AddCorrespondent<'a> {
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[derive(Template)]
#[template(path = "correspondents/index.html")]
pub struct Correspondents<'a>{
    correspondents: Vec<Correspondent>,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[derive(FromForm)]
pub struct AddCorrespondentForm {
    name: String
}

#[get("/correspondents/add")]
pub fn correspondent_add<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>) -> AddCorrespondent<'a> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    AddCorrespondent{rh, current_path}
}

#[post("/correspondents/add", data="<data>")]
pub fn correspondent_add_post<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>, data: Form<AddCorrespondentForm>) -> Redirect {
    println!("Your input: {}", data.get().name);
    add_correspondent(&rh.pool, &(data.get().name));
    Redirect::to(MODULE_PATH)
}

#[get("/correspondents/<id>")]
pub fn correspondent_single<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>, id: i32) -> Option<SingleCorrespondent<'a>> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    let correspondent = fetch_correspondent(&rh.pool, id);
    let documents = fetch_documents_by_correspondent(&rh.pool, id);

    correspondent.map(|corr| SingleCorrespondent { correspondent: corr, documents, rh, current_path })
}

#[get("/correspondents/delete/<id>")]
pub fn correspondent_delete<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>, id: i32) -> Redirect {
    remove_correspondent(&rh.pool, id);
    Redirect::to(MODULE_PATH)
}

#[get("/correspondents")]
pub fn index<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>) -> Correspondents<'a> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    let mut correspondents : Vec<Correspondent> = Vec::new();

    let correspondents = fetch_correspondents(&rh.pool);
    Correspondents { correspondents, rh, current_path }
}