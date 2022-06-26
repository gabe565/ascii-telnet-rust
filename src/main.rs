extern crate log;

use std::env;

use async_std::net::{TcpListener, TcpStream};
use async_std::task::spawn;
use env_logger::Env;
use futures::StreamExt;
use log::{error, info};

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

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            match stream {
                Ok(stream) => {
                    spawn(handle_connection(stream));
                }
                Err(e) => error!("{}", e),
            }
        })
        .await;

    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    MovieClient::new(stream).run().await
}