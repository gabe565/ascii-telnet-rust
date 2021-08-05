use async_std::net::TcpStream;
use async_std::task::sleep;
use futures::AsyncWriteExt;
use log::info;
use std::time::Duration;
use termion::cursor;

const FRAME_HEIGHT: u16 = 13;
const HEIGHT: u16 = 14;

const PAD_LEFT: u16 = 6;
const PAD_TOP: u16 = 4;

const MOVIE: &str = include_str!("../movies/sw1.txt");

pub struct MovieClient {
    stream: TcpStream,
}

impl MovieClient {
    pub fn new(stream: TcpStream) -> Self {
        MovieClient {
            stream,
        }
    }

    async fn clear(&mut self) -> async_std::io::Result<usize> {
        self.stream.write(format!("{}", termion::clear::All).as_bytes()).await
    }

    async fn write_at(&mut self, y: u16, text: &'static str) -> std::io::Result<usize> {
        self.stream.write(
            format!(
                "{}{}\n", cursor::Goto(PAD_LEFT, y + PAD_TOP), text
            ).as_bytes()
        ).await
    }

    async fn log_connect(&mut self) {
        if let Ok(sock) = self.stream.local_addr() {
            info!("Client connected: {}", sock.ip());
        }
    }

    async fn log_disconnect(&mut self) {
        if let Ok(sock) = self.stream.local_addr() {
            info!("Client disconnected: {}", sock.ip());
        }
    }

    async fn log_finish(&mut self) {
        if let Ok(sock) = self.stream.local_addr() {
            info!("Movie finished: {}", sock.ip());
        }
    }

    pub async fn stream(&mut self) {
        if self.clear().await.is_err() {
            return;
        };
        self.log_connect().await;
        let mut sleep_time: u64 = 0;
        for (i, line) in MOVIE.split("\n").enumerate() {
            match (i as u16) % HEIGHT {
                0 => sleep_time = line.parse::<u64>().unwrap() * 1000 / 15,
                curr_line => {
                    if self.write_at(curr_line, line).await.is_err() {
                        self.log_disconnect().await;
                        return;
                    }
                    if curr_line == FRAME_HEIGHT {
                        sleep(Duration::from_millis(sleep_time)).await;
                        if self.clear().await.is_err() {
                            self.log_disconnect().await;
                            return;
                        }
                    }
                }
            }
        }
        self.log_finish().await;
    }
}