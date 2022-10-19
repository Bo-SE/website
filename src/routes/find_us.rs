use rocket_dyn_templates::context;
use rocket_include_tera::{TeraContextManager, EtagIfNoneMatch, TeraResponse};
use rocket::State;

#[get("/find_us")]
pub fn find_us(tera_cm: &State<TeraContextManager>, etag_if_none_match: EtagIfNoneMatch) -> TeraResponse {
    tera_response!(disable_minify tera_cm, etag_if_none_match, "find-us", context! {
        section: "find_us"
    })
}