use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use serde;
use serde_json;
use shellexpand;

use super::Error;

static CONFIG_DEFAULT_PATH: &str = "~/.lightsocks.json";
const CONFIG_ENV_KEY: &str = "LIGHT_SOCKS_CONFIG";
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

fn load_config_by_path(path: &str) -> Config {
    let path = shellexpand::full(path).unwrap().as_ref().to_string();
    let file = fs::File::open(Path::new(&path)).expect("open config failed");
    serde_json::from_reader(file).expect("parse config failed")
}

fn dump_default_config() -> Result<(), Error> {
    let path = shellexpand::full(CONFIG_DEFAULT)
        .unwrap()
        .as_ref()
        .to_string();
    let mut file = fs::File::create(path).expect("open config failed");
    file.write_all(CONFIG_DEFAULT.as_bytes())
        .expect("write config failed");
    Ok(())
}

fn exist_file(path: &str) -> bool {
    let path = shellexpand::full(path).unwrap().as_ref().to_string();
    let path = Path::new(&path);
    path.is_file()
}

pub fn load_config() -> Result<Config, Error> {
    let f = match env::var(CONFIG_ENV_KEY) {
        Ok(v) => v,
        Err(_) => {
            if !exist_file(CONFIG_DEFAULT_PATH) {
                dump_default_config().unwrap();
            }
            CONFIG_DEFAULT_PATH.to_string()
        }
    };

    Ok(load_config_by_path(&f))
}
