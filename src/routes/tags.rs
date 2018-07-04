use rocket::State;
use routes::RoutesHandler;
use std::fs::File;
use models::tag::Tag;
use handlers::taghandler::fetch_tags;
use askama::Template;
use std::sync::Arc;
use fairings::pathfairing::RocketPath;

#[derive(Template)]
#[template(path = "tags.html")]
pub struct Tags<'a> {
    tags: Vec<Tag>,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[get("/tags")]
pub fn index<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>) -> Tags<'a> {
    let current_path : String = (*(path.path.lock().unwrap())).clone();
    let tags = fetch_tags(&rh.pool);
    Tags { tags , rh, current_path }
}