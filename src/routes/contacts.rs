use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};

#[derive(Serialize)]
struct Contact {
    name: &'static str,
    email: &'static str,
    // Other stuff
}

#[get("/contacts")]
pub fn contact_page() -> Template {

    // Forse c'è un modo migliore per passare più contatti
    // Maybe con la struttura Contact e un'altra struttura di supporto contenente i 4 contatti

    let simpaticoni = (
        Contact{
            name: "Filippo De Grandi",
            email: "filippodegrandi02@gmail.com"
        },
        Contact{
            name: "Lorenzo Pattaro Zonta",
            email: "lorenzo.bodini.private@gmail.com"
        },
        Contact{
            name: "Lorenzo Bodini",
            email: "lorenzo.bodini.private@gmail.com"
        },
        Contact{
            name: "Lorenzo Marogna",
            email: "lmarogna02@gmail.com"
        }
    );



    Template::render("contact", context! {
        contacts: simpaticoni
    })
}