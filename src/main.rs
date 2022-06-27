extern crate log;

use std::env;

use async_std::net::{TcpListener, TcpStream};
use async_std::task::spawn;
use env_logger::Env;
use futures::StreamExt;
use log::info;

use crate::movie_client::MovieClient;

mod movie_client;
mod movie;
mod signal_handler;

const DEFAULT_ADDR: &str = "0.0.0.0:23";

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    signal_handler::run()?;

    let addr = env::args().nth(1).unwrap_or(DEFAULT_ADDR.to_string());

    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on {}", &addr);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        spawn(handle_connection(stream?));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    MovieClient::new(stream).run().await
}