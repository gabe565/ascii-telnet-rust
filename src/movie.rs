use once_cell::sync::Lazy;

pub const MOVIE_STR: &str = include_str!("../movies/sw1.txt");

pub static NUM_LINES: Lazy<usize> = Lazy::new(|| {
    MOVIE_STR.split("\n").count()
});

pub const FRAME_HEIGHT: u16 = 13;
pub const HEIGHT: u16 = 14;
pub const WIDTH: u16 = 67;

pub const PAD_X: u16 = 6;
pub const PAD_Y: u16 = 3;