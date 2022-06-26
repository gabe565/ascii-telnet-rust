use std::thread;
use log::info;

use signal_hook::{consts::SIGHUP, iterator::Signals};

use crate::movie_client;

pub fn run() -> anyhow::Result<()> {
    let mut signals = Signals::new(&[SIGHUP])?;

    thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGHUP => info!(
                    "Active connections: {}",
                    unsafe { movie_client::ACTIVE }
                ),
                _ => {},
            }
        }
    });

    Ok(())
}