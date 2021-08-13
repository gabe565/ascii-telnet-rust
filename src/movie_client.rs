use std::time::{Duration, SystemTime};

use async_std::net::TcpStream;
use async_std::task::sleep;
use futures::AsyncWriteExt;
use log::info;
use nanoid::nanoid;
use termion::color;

use crate::movie;

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
        let whole_width = percent * (movie::WIDTH as f32);
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
        let pad = "\n".repeat(movie::PAD_Y as usize);
        format!(
            "{}{}{}[{}{}{}]{}{}",
            pad,
            " ".repeat((movie::PAD_X - 1) as usize),
            color::Fg(color::LightBlack),
            "█".repeat(whole_width as usize),
            part_char,
            " ".repeat((movie::WIDTH as usize) - (whole_width as usize) - 1),
            color::Fg(color::Reset),
            pad,
        )
    }

    async fn stream(&mut self) -> async_std::io::Result<()> {
        self.stream.write_all("\n".repeat(movie::PAD_Y as usize).as_bytes()).await?;
        let mut sleep_time: u64 = 0;
        let mut buffer = Vec::with_capacity(movie::HEIGHT as usize);
        for (i, line) in movie::MOVIE_STR.split("\n").enumerate() {
            match (i as u16) % movie::HEIGHT {
                0 => sleep_time = line.parse::<u64>().unwrap() * 1000 / 15,
                curr_line => {
                    buffer.push(format!(
                        "{}{}\n",
                        " ".repeat(movie::PAD_X as usize),
                        line.to_string()
                    ));
                    if curr_line == movie::FRAME_HEIGHT {
                        buffer.push(Self::progress_bar(
                            i, *movie::NUM_LINES
                        ));
                        self.stream.write_all(buffer.concat().as_bytes()).await?;
                        buffer.clear();
                        sleep(Duration::from_millis(sleep_time)).await;
                        buffer.push(format!(
                            "{}{}",
                            termion::cursor::Up(movie::FRAME_HEIGHT + movie::PAD_Y * 2),
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