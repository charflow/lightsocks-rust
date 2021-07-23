#![deny(unused_imports)]
use common::*;
use serde_json;

fn main() {
    println!("client for lightsocks: version {}", VERSION);
    if !exist_file(CONFIG_DEFAULT_PATH) {
        dump_default_config(CONFIG_DEFAULT_PATH).unwrap();
        println!("{} does not exist, generated.", CONFIG_DEFAULT_PATH);
    }
    let cfg = load_config(CONFIG_DEFAULT_PATH);
    println!("config: {}", serde_json::to_string_pretty(&cfg).unwrap());
}
