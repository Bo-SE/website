use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};

#[derive(Serialize)]
struct Contact {
    name: String,
    email: String,
    // Other stuff
}

#[get("/contacts")]
pub fn contact_page() -> Template {

    // Forse c'è un modo migliore per passare più contatti
    // Maybe con la struttura Contact e un'altra struttura di supporto contenente i 4 contatti

    Template::render("contact", context! {
        n1: "Filippo De Grandi",
        e1: "filippodegrandi02@gmail.com"
        // Aggiungete i vostri
    })
}