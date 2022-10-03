#![feature(decl_macro)]
use serde::Serialize;
#[macro_use] extern crate rocket;
use std::fmt::format;
use rocket::{Build, Rocket};
use rocket_contrib::templates::Template;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, hello])
        .attach(Template::fairing())
}

#[derive(Serialize)]
struct Person {
    name: String,
    surname: String
}

#[get("/")]
fn index() -> Template {
    let content = Person { name: "Lore".to_string(), surname: "Lor".to_string() };
    Template::render("index", content)
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello {}!", name)
}