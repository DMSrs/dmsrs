use routes::RoutesHandler;
use rocket::State;
use rocket_contrib::Template;
use tera::Context;

#[get("/")]
pub fn index(rh: State<RoutesHandler>) -> Template {
    let mut context = Context::new();
    Template::render("index", &context)
}