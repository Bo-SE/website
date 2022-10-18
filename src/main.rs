#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_include_tera;

extern crate rocket_dyn_templates;
use rocket::{Build, Rocket};
use rocket::fs::FileServer;
use rocket_include_tera::{TeraResponse};


mod routes;

// using all routes from the "routes" directory
//main functions for rocket server
#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(TeraResponse::fairing(|tera|{
            tera_resources_initialize!(
                tera,
                "base" => "templates/base.html",
                "graphics" => "templates/graphics.html",
                "index" => "templates/index.html",
                "header" => "templates/header.html",
                "contacts" => "templates/contacts.html",
                "find-us" => "templates/find-us.html"
            )
        }))
        .mount("/static", FileServer::from("templates/static"))
        .mount("/", routes![ //list of routes
            routes::index::homepage,
            routes::contacts::contact_page,
            routes::contacts::contacts_post,
            routes::find_us::find_us,
        ])
}



