use rocket::response::Redirect;

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::moved("/static/favicon.ico")
}
