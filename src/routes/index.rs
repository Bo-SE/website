use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};
use crate::routes::trentino_api::Busses;

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
    // this will contain all the busses, then render the vector contained in the struct
    let busses: Busses =  crate::routes::trentino_api::parse_json("").unwrap();

    let context = Person { name: "Borino".to_string(), surname: "Stock Exchange".to_string() };

    // let info_banner = (
    //         Bus{
    //             bus_stop: "PVVLN",
    //             bus_info: "5 min ritardo",
    //             is_late: true
    //         },
    //         Bus{
    //             bus_stop: "PVSL",
    //             bus_info: "In orario",
    //             is_late: false
    //         },
    //         Bus{
    //             bus_stop: "PVFCSCNZ",
    //             bus_info: "10 min ritardo",
    //             is_late: true
    //         },
    //         Bus{
    //             bus_stop: "PVPZMNC",
    //             bus_info: "5 min ritardo",
    //             is_late: true
    //         },
    //         Bus{
    //             bus_stop: "PVPNT",
    //             bus_info: "In orario",
    //             is_late: false
    //         },
    //         Bus{
    //             bus_stop: "STZNFS",
    //             bus_info: "2 minuti ritardo",
    //             is_late: true
    //         }
    //     );

    let template = Template::render("index", context! {
        index: busses
    });
    template
}

