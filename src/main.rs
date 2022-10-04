#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

mod routes; // using all routes from the "routes" directory

#[launch]
fn rocket() -> Rocket<Build>{
    rocket::build()
        .mount("/", routes![ //list of routes
            routes::index::homepage,
            routes::index::contact_page
            // all the others
        ])
        .attach(Template::fairing())
}

