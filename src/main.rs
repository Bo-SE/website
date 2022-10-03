#![feature(decl_macro)]
#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use serde::Serialize;

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}

#[derive(Serialize)]
struct Person {
    name: String,
    surname: String
}

#[get("/index")]
fn index() -> Template {
    let content = Person { name: "Lore".to_string(), surname: "Lor".to_string() };
    Template::render("index", content)
}