use std::{process, thread};

use log::info;
use signal_hook::{consts::SIGHUP, consts::SIGINT, consts::SIGQUIT, consts::SIGTERM, iterator::Signals};

use crate::movie_client;

pub fn run() -> anyhow::Result<()> {
    let mut signals = Signals::new(&[SIGHUP, SIGINT, SIGQUIT, SIGTERM])?;

    thread::spawn(move || {
        for signal in signals.forever() {
            match signal {
                SIGHUP => info!(
                    "Active connections: {}",
                    unsafe { movie_client::ACTIVE }
                ),
                SIGINT | SIGQUIT | SIGTERM => process::exit(130),
                _ => {},
            }
        }
    });

    Ok(())
}