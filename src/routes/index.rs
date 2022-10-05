use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};
use crate::routes::trentino_api::Busses;

//request of templates/index.html.hbs
#[get("/")]
pub fn homepage() -> Template {
    let json = r#"[
        {
            "stop_name": "PVVLN",
            "stop_time": "10:50",
            "time_delta": 5,
            "in_time": false
        },
        {
            "stop_name": "PVSL",
            "stop_time": "10:35",
            "time_delta": 0,
            "in_time": true
        },
        {
            "stop_name": "PVFCSCNZ",
            "stop_time": "10:40",
            "time_delta": 0,
            "in_time": true
        },
        {
            "stop_name": "PVPNT",
            "stop_time": "10:40",
            "time_delta": 3,
            "in_time": false
        },
        {
            "stop_name": "STZNFS",
            "stop_time": "10:20",
            "time_delta": 10,
            "in_time": false
        }
    ]
    "#;
    // this will contain all the busses, then render the vector contained in the struct
    let busses_struct: Busses =  crate::routes::trentino_api::parse_json(json).unwrap();

    println!("{:?}", busses_struct);

    Template::render("index", context! {
        index: busses_struct.busses,
        name: "Borino",
        surname: "Stock Exchange"
    })
}

