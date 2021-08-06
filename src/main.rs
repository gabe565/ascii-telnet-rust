extern crate log;

use async_std::net::TcpListener;
use env_logger::Env;
use futures::StreamExt;
use log::{error, info};

use crate::movie_client::MovieClient;

mod movie_client;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let host = "0.0.0.0";
    let port = "23";
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on {}", &addr);

    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            match stream {
                Ok(stream) => {
                    MovieClient::new(stream).run().await;
                }
                Err(e) => error!("{}", e),
            }
        })
        .await;

    Ok(())
}
