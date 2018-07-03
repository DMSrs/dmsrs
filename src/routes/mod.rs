
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use models::menuentry::MenuEntry;
use rocket::fairing::Fairing;
use rocket::Rocket;
use rocket::Request;
use rocket::Data;
use rocket::fairing::Info;
use rocket::fairing::Kind;
use atomic::Atomic;
use atomic::Ordering;
use rocket::http::uri::URI;
use std::sync::Arc;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

pub mod documents;
pub mod index;
pub mod tags;

pub struct RoutesHandler {
    pub pool: Pool<PostgresConnectionManager>,
    pub current_page: Mutex<String>,
    pub menu: Vec<MenuEntry>
}

impl Fairing for RoutesHandler {
    fn info(&self) -> Info {
        Info {
            name: "RoutesHandler",
            kind: Kind::Request | Kind::Attach
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        Ok(rocket.manage(self))
    }


    fn on_request(&self, request: &mut Request, data: &Data) {
        let mut data = self.current_page.lock().unwrap();
        *data = String::from(request.uri().path());
    }
}
