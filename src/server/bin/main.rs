#![deny(unused_imports)]
use common::*;
use serde_json;

fn main() {
    println!("server for lightsocks: version {}", VERSION);
    let cfg = load_config().unwrap();
    println!("config: {}", serde_json::to_string_pretty(&cfg).unwrap());
}
