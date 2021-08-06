use std::time::Duration;

use async_std::net::TcpStream;
use async_std::task::sleep;
use futures::AsyncWriteExt;
use log::info;
use termion::cursor;

const FRAME_HEIGHT: u16 = 13;
const HEIGHT: u16 = 14;

const PAD_LEFT: u16 = 6;
const PAD_TOP: u16 = 4;

const MOVIE: &str = include_str!("../movies/sw1.txt");

const LOG_CONNECT: &str = "Client connected";
const LOG_DISCONNECT: &str = "Client disconnected";
const LOG_FINISH: &str = "Movie finished";

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

    pub async fn stream(&mut self) {
        if self.clear().await.is_err() {
            return;
        };
        info!("{}", LOG_CONNECT);
        let mut sleep_time: u64 = 0;
        let mut buffer = Vec::with_capacity(HEIGHT as usize);
        for (i, line) in MOVIE.split("\n").enumerate() {
            match (i as u16) % HEIGHT {
                0 => sleep_time = line.parse::<u64>().unwrap() * 1000 / 15,
                curr_line => {
                    buffer.push(format!(
                        "{}{}\n",
                        cursor::Goto(PAD_LEFT, curr_line + PAD_TOP),
                        line.trim_end(),
                    ));
                    if curr_line == FRAME_HEIGHT {
                        if self.stream.write(buffer.concat().as_bytes()).await.is_err() {
                            info!("{}", LOG_DISCONNECT);
                            return;
                        }
                        buffer.clear();
                        buffer.push(format!("{}", termion::clear::All));
                        sleep(Duration::from_millis(sleep_time)).await;
                    }
                }
            }
        }
        info!("{}", LOG_FINISH);
    }
}