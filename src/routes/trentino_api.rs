use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bus {
    stop_name: String,
    stop_time: String,
    time_delta: i32,
    in_time: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Busses {
    busses: Vec<Bus>,
}

pub fn parse_json(json: &str) -> Result<(Busses)> {
    let mut parsed_json: Value = serde_json::from_str(json)?;
    let busses = parsed_json.as_array().unwrap();
    let mut parsed_struct = Busses {busses: Vec::new()};

    for bus in busses {
        let b = Bus {
            stop_name: bus["stop_name"].to_string(),
            stop_time: bus["stop_time"].to_string(),
            time_delta: 0, // to Calculate
            in_time: bus["in_time"].as_bool().unwrap(),
        };

        parsed_struct.busses.push(b);
    }

    println!("{:?}", busses);

    Ok((parsed_struct))
}
