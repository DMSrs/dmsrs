use rocket::State;
use routes::RoutesHandler;
use std::fs::File;
use rocket_contrib::Template;
use tera::Context;
use models::tag::Tag;
use handlers::taghandler::fetch_tags;

#[get("/tags")]
pub fn index(rh: State<RoutesHandler>) -> Template {
    let mut context = Context::new();
    let tags = fetch_tags(&rh.pool);

    context.add("tags", &tags);

    Template::render("tags", &context)
}