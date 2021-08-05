use crate::movie_client::MovieClient;
use std::net::TcpListener;

mod movie_client;

fn main() {
    //TODO: Dynamic config
    let host = "0.0.0.0";
    let port = "23";
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on {}", &addr);

    while let Ok((stream, sock)) = listener.accept() {
        println!("Connection from {}", sock.ip());
        MovieClient::new(stream).begin();
    }
}
