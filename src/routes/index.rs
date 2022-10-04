use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};

//derive of the serialize trait
#[derive(Serialize)]
struct Person {
    name: String,
    surname: String
}

#[derive(Serialize)]
struct Bus {
    bus_stop: &'static str,
    bus_info: &'static str,
    is_late: bool
    // Other stuff
}

//request of templates/index.html.hbs
#[get("/")]
pub fn homepage() -> Template {
    let context = Person { name: "Borino".to_string(), surname: "Stock Exchange".to_string() };
    let info_banner = (
            Bus{
                bus_stop: "PVVLN",
                bus_info: "5 min ritardo",
                is_late: true
            },
            Bus{
                bus_stop: "PVSL",
                bus_info: "In orario",
                is_late: false
            },
            Bus{
                bus_stop: "PVFCSCNZ",
                bus_info: "10 min ritardo",
                is_late: true
            },
            Bus{
                bus_stop: "PVPZMNC",
                bus_info: "5 min ritardo",
                is_late: true
            },
            Bus{
                bus_stop: "PVPNT",
                bus_info: "In orario",
                is_late: false
            },
            Bus{
                bus_stop: "STZNFS",
                bus_info: "2 minuti ritardo",
                is_late: true
            }
        );

    Template::render("index", context! {
        index: info_banner
    })
}

