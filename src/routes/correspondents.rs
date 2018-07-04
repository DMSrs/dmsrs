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

#[derive(Template)]
#[template(path = "correspondent.html")]
pub struct SingleCorrespondent<'a> {
    correspondent: Correspondent,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[derive(Template)]
#[template(path = "correspondents.html")]
pub struct Correspondents<'a>{
    correspondents: Vec<Correspondent>,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[get("/correspondents/<id>")]
pub fn correspondent_single<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>, id: i32) -> Option<SingleCorrespondent<'a>> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    let correspondent = fetch_correspondent(&rh.pool, id);

    correspondent.map(|corr| SingleCorrespondent { correspondent: corr, rh, current_path })
}

#[get("/correspondents")]
pub fn index<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>) -> Correspondents<'a> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    let mut correspondents : Vec<Correspondent> = Vec::new();

    let correspondents = fetch_correspondents(&rh.pool);
    Correspondents { correspondents, rh, current_path }
}