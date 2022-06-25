use signal_hook::{consts::SIGHUP, iterator::Signals};
use std::{error::Error, thread};
use crate::movie_client;

pub fn handle_signals() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGHUP])?;

    thread::spawn(move || unsafe {
        for signal in signals.forever() {
            match signal {
                SIGHUP => {
                    println!("Active connections: {}", movie_client::ACTIVE)
                }
                _ => {}
            }
        }
    });

    Ok(())
}