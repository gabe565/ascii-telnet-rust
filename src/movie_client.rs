use std::net::TcpStream;
use termion::cursor;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

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

    fn clear(&mut self) -> std::io::Result<usize> {
        self.write(format!("{}", termion::clear::All).as_bytes())
    }

    fn write_at(&mut self, y: u16, text: &'static str) -> std::io::Result<usize> {
        self.write(
            format!(
                "{}{}\n", cursor::Goto(PAD_LEFT, y + PAD_TOP), text
            ).as_bytes()
        )
    }

    pub fn begin(&mut self) {
        let mut sleep_time: u64 = 0;
        for (i, line) in MOVIE.split("\n").enumerate() {
            let curr_line = (i as u16) % HEIGHT;
            if curr_line == 0 {
                sleep_time = line.parse::<u64>().unwrap() * 1000 / 15;
            } else {
                self.write_at(curr_line, line).unwrap();
                if curr_line == FRAME_HEIGHT {
                    sleep(Duration::from_millis(sleep_time));
                    self.clear().unwrap();
                }
            }
        }
    }
}

impl Write for MovieClient {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}