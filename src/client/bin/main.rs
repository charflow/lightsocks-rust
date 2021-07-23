#![deny(unused_imports)]
use common::*;
use serde_json;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_socks::{tcp::Socks5Stream, Error};

#[tokio::main]
pub async fn main() {
    println!("client for lightsocks: version {}", VERSION);
    let cfg = load_config().unwrap();
    println!("config: {}", serde_json::to_string_pretty(&cfg).unwrap());

    // start proxy
    send_to_server(&cfg, 0).await.unwrap();
}

async fn send_to_server(cfg: &Config, _req: TODO) -> Result<(), Error> {
    const DEST_ADDR: &str = "google.com:80";

    let tcp_stream = TcpStream::connect(&cfg.remote).await?;
    let mut socks5_stream = Socks5Stream::connect_with_socket(tcp_stream, DEST_ADDR).await?;

    socks5_stream.write_all(b"GET /\n\n").await?;
    let mut buf = Vec::new();
    let n = socks5_stream.read_to_end(&mut buf).await?;
    println!("{} bytes read\n\n{}", n, String::from_utf8_lossy(&buf));
    Ok(())
}
