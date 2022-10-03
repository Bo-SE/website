#[macro_use] extern crate rocket;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

mod routes; // using all routes from the "routes" directory

#[launch]
fn rocket() -> Rocket<Build>{
    rocket::build()
        .mount("/", routes![ //list of routes
            routes::index::homepage,
            // all the others
        ])
        .attach(Template::fairing())
}

