use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};

//derive of the serialize trait
#[derive(Serialize)]
struct Person {
    name: String,
    surname: String
}

//request of templates/index.html.hbs
#[get("/")]
pub fn homepage() -> Template {
    let context = Person { name: "Borino".to_string(), surname: "Stock Exchange".to_string() };
    Template::render("index", &context)
}

#[get("/contactUs")]
pub fn contact_page()-> Template{
    let context = Person { name: "mother".to_string(), surname :"fucker".to_string()};
    Template::render("contact", &context)
}
