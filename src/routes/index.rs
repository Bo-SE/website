use rocket_include_tera::{TeraContextManager, EtagIfNoneMatch, TeraResponse};
use rocket::State;
use rocket_dyn_templates::context;

//request of templates/index.html
#[get("/")]
pub fn homepage(tera_cm: &State<TeraContextManager>, etag_if_none_match: EtagIfNoneMatch) -> TeraResponse {
    tera_response!(disable_minify tera_cm, etag_if_none_match, "index", context! {
        title: "Welcome",
        section: "home"
    })
}

