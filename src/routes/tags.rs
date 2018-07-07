use rocket::State;
use routes::RoutesHandler;
use std::fs::File;
use models::tag::Tag;
use handlers::taghandler::fetch_tags;
use askama::Template;
use std::sync::Arc;
use fairings::pathfairing::RocketPath;
use models::document::Document;
use handlers::taghandler::fetch_tag;
use handlers::documenthandler::fetch_documents_by_tag;

#[derive(Template)]
#[template(path = "tags/index.html")]
pub struct Tags<'a> {
    tags: Vec<Tag>,
    rh: State<'a, RoutesHandler>,
    current_path: String
}

#[derive(Template)]
#[template(path = "tags/single.html")]
pub struct SingleTag<'a> {
    tag: Tag,
    documents: Vec<Document>,
    rh: State<'a, RoutesHandler>,
    current_path: String
}


#[get("/tags")]
pub fn index<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>) -> Tags<'a> {
    let current_path : String = (*(path.path.lock().unwrap())).clone();
    let tags = fetch_tags(&rh.pool);
    Tags { tags , rh, current_path }
}

#[get("/tags/<slug>")]
pub fn tag_single<'a>(rh: State<'a, RoutesHandler>, path: State<Arc<RocketPath>>, slug: String) -> Option<SingleTag<'a>> {
    let mut current_path : String = (*(path.path.lock().unwrap())).clone();
    let tag = fetch_tag(&rh.pool, slug.clone());
    return match tag {
        Some(t) => {
            let documents_by_tag = fetch_documents_by_tag(&rh.pool, slug.clone());
            Some(SingleTag { tag: t, documents: documents_by_tag, rh, current_path })
        }
        None => {
           None
        }
    }

}