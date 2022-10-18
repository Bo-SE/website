use rocket::serde::Serialize;
use rocket_dyn_templates::context;
use rocket_include_tera::{TeraContextManager, EtagIfNoneMatch, TeraResponse};
use rocket::State;
use rocket::form::Form;
use rocket::form::FromForm;

#[derive(Serialize)]
struct Contact {
    name: &'static str,
    email: &'static str,
    // Other stuff
}

#[get("/contacts")]
pub fn contact_page(tera_cm: &State<TeraContextManager>, etag_if_none_match: EtagIfNoneMatch) -> TeraResponse {
    let simpaticoni = (
        Contact{
            name: "Filippo De Grandi",
            email: "filippodegrandi02@gmail.com"
        },
        Contact{
            name: "Lorenzo Pattaro Zonta",
            email: "lpzonta@gmail.com"
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

    tera_response!(tera_cm, etag_if_none_match, "contacts", context!{
        contacts: simpaticoni,
        title: "Contacts"
    })
}

#[derive(Debug, FromForm, Serialize)]
pub struct Task<'r> {
    description: &'r str,
}

#[post("/contacts", data = "<task>")]
pub fn contacts_post(task: Form<Task<'_>>, tera_cm: &State<TeraContextManager>, etag_if_none_match: EtagIfNoneMatch) -> TeraResponse {
    let simpaticoni = (
        Contact{
            name: "Filippo De Grandi",
            email: "filippodegrandi02@gmail.com"
        },
        Contact{
            name: "Lorenzo Pattaro Zonta",
            email: "lpzonta@gmail.com"
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


    tera_response!(tera_cm, etag_if_none_match, "contacts", context! {
        contacts: simpaticoni,
        name: "Borino",
        title: "Contacts",
        surname: "Stock Exchange",
        task: &*task
    })
}