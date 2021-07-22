#![deny(unused_imports)]
use common::*;
use serde_json;

fn main() {
    println!("server for lightsocks: version {}", VERSION);
    if !exist_file(CONFIG_DEFAULT_PATH) {
        let _ = dump_default_config(CONFIG_DEFAULT_PATH);
        println!("{} does not exist, generated.", CONFIG_DEFAULT_PATH);
    }
    let cfg = load_config("~/.lightsocks.json");
    println!("config: {}", serde_json::to_string_pretty(&cfg).unwrap());
    let _ = dump_config("server.json", &cfg);
}
