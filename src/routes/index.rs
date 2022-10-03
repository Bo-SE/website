use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct Person {
    name: String,
    surname: String
}

#[get("/")]
pub fn homepage() -> Template {
    let context = Person { name: "Borino".to_string(), surname: "Stock Exchange".to_string() };
    Template::render("index", &context)
}
