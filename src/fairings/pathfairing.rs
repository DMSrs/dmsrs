use rocket::fairing::Fairing;
use std::path::PathBuf;
use rocket::fairing::Info;
use rocket::Rocket;
use rocket::Request;
use rocket::Data;
use rocket::fairing::Kind;
use std::sync::Mutex;
use std::sync::Arc;

pub struct PathFairing {
    pub rp: Arc<RocketPath>
}

pub struct RocketPath {
    pub path: Mutex<String>
}

impl PathFairing {
    pub fn new() -> PathFairing {
        PathFairing{
            rp: Arc::from(RocketPath::new())
        }
    }
}

impl RocketPath {
    pub fn new() -> RocketPath{
        RocketPath {
            path: Mutex::new(String::from("N/A"))
        }
    }
}

impl Fairing for PathFairing {
    fn info(&self) -> Info {
        Info {
            name: "Path Fairing",
            kind: Kind::Attach | Kind::Request
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        Ok(rocket
            .manage(self.rp.clone()))
    }

    fn on_request(&self, request: &mut Request, data: &Data) {
        let mut path = self.rp.path.lock().unwrap();
        *path = String::from(request.uri().path());
    }
}