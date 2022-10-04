#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
use rocket::{Build, Rocket};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod routes; // using all routes from the "routes" directory

//main functions for rocket server
#[launch]
fn rocket() -> Rocket<Build>{
    rocket::build()
        .mount("/", routes![ //list of routes
            routes::index::homepage,
            routes::contacts::contact_page,
            // all the others
        ])
        .mount("/static", FileServer::from("templates/static"))
        .attach(Template::fairing())
}



