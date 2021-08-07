use std::time::{Duration, SystemTime};

use async_std::net::TcpStream;
use async_std::task::sleep;
use futures::AsyncWriteExt;
use log::info;
use nanoid::nanoid;
use termion::{cursor, color};

const FRAME_HEIGHT: u16 = 13;
const HEIGHT: u16 = 14;
const WIDTH: u16 = 67;

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

    fn build_line(x: u16, y: u16, text: String) -> String {
        format!(
            "{}{}\n",
            cursor::Goto(x, y),
            text,
        )
    }

    fn progress_bar(y: u16, line_num: usize, total: usize) -> String {
        let percent = (line_num as f32) / (total as f32);
        let whole_width = percent * (WIDTH as f32);
        let part_char = match (whole_width % 1.0 * 8.0) as u16 {
            0 => " ",
            1 => "▏",
            2 => "▎",
            3 => "▍",
            4 => "▌",
            5 => "▋",
            6 => "▊",
            7 => "▉",
            _ => "█",
        };
        Self::build_line(PAD_LEFT - 1, y, format!(
            "{}[{}{}{}]{}",
            color::Fg(color::LightBlack),
            "█".repeat(whole_width as usize),
            part_char,
            " ".repeat((WIDTH as usize) - (whole_width as usize) - 1),
            color::Fg(color::Reset)
        ))
    }

    async fn stream(&mut self) -> async_std::io::Result<()> {
        self.clear().await?;
        let line_count = MOVIE.split("\n").count();
        let mut sleep_time: u64 = 0;
        let mut buffer = Vec::with_capacity(HEIGHT as usize);
        for (i, line) in MOVIE.split("\n").enumerate() {
            match (i as u16) % HEIGHT {
                0 => sleep_time = line.parse::<u64>().unwrap() * 1000 / 15,
                curr_line => {
                    buffer.push(Self::build_line(
                        PAD_LEFT, curr_line + PAD_TOP, line.to_string()
                    ));
                    if curr_line == FRAME_HEIGHT {
                        buffer.push(Self::progress_bar(
                            curr_line + PAD_TOP + 3, i, line_count
                        ));
                        self.stream.write(buffer.concat().as_bytes()).await?;
                        buffer.clear();
                        sleep(Duration::from_millis(sleep_time)).await;
                        buffer.push(format!("{}", termion::clear::All));
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