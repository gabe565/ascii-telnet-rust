use std::time::{Duration, SystemTime};

use async_std::net::TcpStream;
use async_std::task::sleep;
use futures::AsyncWriteExt;
use log::info;
use nanoid::nanoid;
use termion::color;

const FRAME_HEIGHT: u16 = 13;
const HEIGHT: u16 = 14;
const WIDTH: u16 = 67;

const PAD_X: u16 = 6;
const PAD_Y: u16 = 3;

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

    fn progress_bar(line_num: usize, total: usize) -> String {
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
        let pad = "\n".repeat(PAD_Y as usize);
        format!(
            "{}{}{}[{}{}{}]{}{}",
            pad,
            " ".repeat((PAD_X - 1) as usize),
            color::Fg(color::LightBlack),
            "█".repeat(whole_width as usize),
            part_char,
            " ".repeat((WIDTH as usize) - (whole_width as usize) - 1),
            color::Fg(color::Reset),
            pad,
        )
    }

    async fn stream(&mut self) -> async_std::io::Result<()> {
        self.stream.write("\n".repeat(PAD_Y as usize).as_bytes()).await?;
        let line_count = MOVIE.split("\n").count();
        let mut sleep_time: u64 = 0;
        let mut buffer = Vec::with_capacity(HEIGHT as usize);
        for (i, line) in MOVIE.split("\n").enumerate() {
            match (i as u16) % HEIGHT {
                0 => sleep_time = line.parse::<u64>().unwrap() * 1000 / 15,
                curr_line => {
                    buffer.push(format!(
                        "{}{}\n",
                        " ".repeat(PAD_X as usize),
                        line.to_string()
                    ));
                    if curr_line == FRAME_HEIGHT {
                        buffer.push(Self::progress_bar(
                            i, line_count
                        ));
                        self.stream.write(buffer.concat().as_bytes()).await?;
                        buffer.clear();
                        sleep(Duration::from_millis(sleep_time)).await;
                        buffer.push(format!(
                            "{}{}",
                            termion::cursor::Up(FRAME_HEIGHT + PAD_Y * 2),
                            termion::clear::AfterCursor,
                        ));
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