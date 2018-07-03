use rocket::State;
use routes::RoutesHandler;
use std::fs::File;
use models::tag::Tag;
use handlers::taghandler::fetch_tags;
use askama::Template;

#[derive(Template)]
#[template(path = "tags.html")]
pub struct Tags<'a> {
    tags: Vec<Tag>,
    rh: State<'a, RoutesHandler>
}

#[get("/tags")]
pub fn index(rh: State<RoutesHandler>) -> Tags {
    let tags = fetch_tags(&rh.pool);
    Tags { tags , rh }
}