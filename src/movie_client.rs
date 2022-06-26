use std::time::{Duration, SystemTime};

use async_std::net::TcpStream;
use async_std::task::sleep;
use futures::AsyncWriteExt;
use log::info;
use termion::color;

use crate::movie;

pub static mut ACTIVE: u32 = 0;

pub struct MovieClient {
    connected_at: SystemTime,
    stream: TcpStream,
}

impl MovieClient {
    pub fn new(stream: TcpStream) -> Self {
        MovieClient {
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
        let mut this_sleep: u64 = 0;
        let mut next_sleep: u64 = 0;
        let mut buffer = Vec::with_capacity(movie::HEIGHT as usize);
        for (i, line) in movie::MOVIE_STR.split("\n").enumerate() {
            match (i as u16) % movie::HEIGHT {
                0 => next_sleep = line.parse::<u64>().unwrap() * 1000 / 15,
                curr_line => {
                    buffer.push(format!(
                        "{}{}\n",
                        " ".repeat(movie::PAD_X as usize),
                        line.to_string()
                    ));
                    if curr_line == movie::FRAME_HEIGHT {
                        buffer.push(Self::progress_bar(
                            i, *movie::NUM_LINES,
                        ));
                        sleep(Duration::from_millis(this_sleep)).await;
                        self.stream.write_all(buffer.concat().as_bytes()).await?;
                        buffer.clear();
                        buffer.push(format!(
                            "{}{}",
                            termion::cursor::Up(movie::FRAME_HEIGHT + movie::PAD_Y * 2),
                            termion::clear::AfterCursor,
                        ));
                        this_sleep = next_sleep;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn run(&mut self) {
        unsafe { ACTIVE += 1; }
        let ip = match self.stream.peer_addr() {
            Ok(sock) => sock.ip().to_string(),
            Err(_) => "?".to_string(),
        };
        let result = self.stream().await;
        let elapsed = self.connected_at.elapsed().unwrap();
        match result {
            Ok(_) => info!("{} Finished movie", ip),
            Err(_) => {}
        }
        info!("{} Disconnected after {:.2}s", ip, elapsed.as_secs_f32());
        unsafe { ACTIVE -= 1; }
    }
}