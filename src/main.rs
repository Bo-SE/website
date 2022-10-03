#[macro_use] extern crate rocket;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use rocket::serde::Serialize;

mod get_functions;

#[launch]
fn rocket() -> Rocket<Build>{
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}

#[derive(Serialize)]
struct Person {
    name: String,
    surname: String
}

#[get("/")]
fn index() -> Template {
    let content = Person { name: "Borino".to_string(), surname: "Stock Exchange".to_string() };
    Template::render("index", &content)
}
