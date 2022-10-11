use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};

#[get("/find_us")]
pub fn find_us() -> Template {
    Template::render("find-us", context! {})
}