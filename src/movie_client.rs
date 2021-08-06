use std::time::{Duration, SystemTime};

use async_std::net::TcpStream;
use async_std::task::sleep;
use futures::AsyncWriteExt;
use log::info;
use nanoid::nanoid;
use termion::cursor;

const FRAME_HEIGHT: u16 = 13;
const HEIGHT: u16 = 14;

const PAD_LEFT: u16 = 6;
const PAD_TOP: u16 = 4;

const MOVIE: &str = include_str!("../movies/sw1.txt");

macro_rules! client_log {
    ($id:expr, $text:expr) => {
        info!("[{}] {}", $id, $text);
    }
}

pub struct MovieClient {
    id: String,
    connected_at: SystemTime,
    stream: TcpStream,
}

impl MovieClient {
    pub fn new(stream: TcpStream) -> Self {
        MovieClient {
            id: nanoid!(8),
            connected_at: SystemTime::now(),
            stream,
        }
    }

    async fn clear(&mut self) -> async_std::io::Result<usize> {
        self.stream.write(format!("{}", termion::clear::All).as_bytes()).await
    }

    async fn stream(&mut self) -> async_std::io::Result<()> {
        self.clear().await?;
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
                        self.stream.write(buffer.concat().as_bytes()).await?;
                        buffer.clear();
                        buffer.push(format!("{}", termion::clear::All));
                        sleep(Duration::from_millis(sleep_time)).await;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn run(&mut self) {
        client_log!(self.id, "Connected");
        let result = self.stream().await;
        let elapsed = self.connected_at.elapsed().unwrap();
        match result {
            Ok(_) => client_log!(
                self.id, format!("[{:.2}s] Finished", elapsed.as_secs_f32())
            ),
            Err(_) => client_log!(
                self.id, format!("[{:.2}s] Disconnected", elapsed.as_secs_f32())
            ),
        }
    }
}