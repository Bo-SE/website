use std::collections::HashMap;
use rocket_include_tera::{TeraContextManager, EtagIfNoneMatch, TeraResponse};
use rocket::State;

//request of templates/index.html
#[get("/")]
pub fn homepage(tera_cm: &State<TeraContextManager>, etag_if_none_match: EtagIfNoneMatch) -> TeraResponse {
    let mut c = HashMap::new();
    c.insert("var", (50 + 2).to_string());
    c.insert("title", "Welcome".to_string());
    //t.render("text.html.tera", &c).unwrap()
    tera_response!(tera_cm, etag_if_none_match, "index", c)
}

