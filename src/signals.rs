use std::thread;

use signal_hook::{consts::SIGHUP, iterator::Signals};

use crate::movie_client;

pub fn handle_signals() -> anyhow::Result<()> {
    let mut signals = Signals::new(&[SIGHUP])?;

    thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGHUP => println!(
                    "Active connections: {}",
                    unsafe { movie_client::ACTIVE }
                ),
                _ => {},
            }
        }
    });

    Ok(())
}