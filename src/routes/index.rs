use routes::RoutesHandler;
use rocket::State;
use tera::Context;
use fairings::pathfairing::RocketPath;
use std::sync::Arc;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Home<'a> {
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[get("/")]
pub fn index<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>) -> Home<'a> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    Home { rh, current_path }
}