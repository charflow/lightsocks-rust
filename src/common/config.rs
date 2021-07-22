use std::fs;
use std::path::Path;

use serde;
use serde_json;
use shellexpand;

use super::Error;

pub static CONFIG_DEFAULT_PATH: &str = "~/.lightsocks.json";
static CONFIG_DEFAULT: &str = r#"{
    "listen": ":7448",
    "remote": "",
    "password": ""
}"#;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub listen: String,
    pub remote: String,
    pub password: String,
}

pub fn load_config(path: &str) -> Config {
    let path = shellexpand::full(path).unwrap().as_ref().to_string();
    let file = fs::File::open(Path::new(&path)).expect("open config failed");
    serde_json::from_reader(file).expect("parse config failed")
}

pub fn dump_config(path: &str, cfg: &Config) -> Result<(), Error> {
    let path = shellexpand::full(path).unwrap().as_ref().to_string();
    let file = fs::File::create(path).expect("open config failed");
    let _ = serde_json::to_writer_pretty(file, cfg);
    Ok(())
}

pub fn dump_default_config(path: &str) -> Result<(), Error> {
    let path = shellexpand::full(path).unwrap().as_ref().to_string();
    let file = fs::File::create(path).expect("open config failed");
    let _ = serde_json::to_writer(file,v);
    Ok(())
}

pub fn exist_file(path: &str) -> bool {
    let path = shellexpand::full(path).unwrap().as_ref().to_string();
    let path = Path::new(&path);
    path.is_file()
}
