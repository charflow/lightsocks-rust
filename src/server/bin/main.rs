#![deny(unused_imports)]
use common::*;
use serde_json;
use anyhow::Result;
use socks5_proxy::server;

#[tokio::main]
async fn main() -> Result<()> {
    println!("server for lightsocks: version {}", VERSION);
    let cfg = load_config().unwrap();
    println!("config: {}", serde_json::to_string_pretty(&cfg)?);

    let s = server::new(cfg.listen.parse()?, None)?;
    s.run().await?;
    Ok(())
}