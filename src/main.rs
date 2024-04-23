//! This file contains all driver code for the program.
use crate::print_players::print_players;
use crate::print_text::print_text;
use crate::update_message::update_message;
use crate::update_players::update_players;
use clap::Parser;
use core::time;
use log::{error, info, warn};
use mpris::PlayerFinder;
use std::ffi::OsString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use structs::cli::Cli;
use structs::{config::Config, data::Data};

mod print_players;
mod print_text;
mod structs;
mod update_message;
mod update_players;

/// This function deals with an incoming (USR1) signal.
/// It is hard-coded to play/pause the active player.
///
/// input:
/// data: Data struct for active configuration.
fn handle_signal(data: &Data) {
    if let Some(p) = &data.current_player {
        match p.checked_play_pause() {
            Ok(b) => match b {
                true => info!("Player play/paused succesfully!"),
                false => warn!("Failed to send play/pause signal!"),
            },
            Err(e) => error!("{e}"),
        }
    }
}

/// This function contains the default maim loop body of the program.
/// It updates the active player, updates the output strings based on this, and finally formats and outputs these strings to stdout.
///
/// input:
/// pf: PlayerFinder instance for the program
/// cfg: Configuration of the program
/// data: mutable Data struct, active state of the program
/// r: pre-computed rating strings
fn default_loop(pf: &PlayerFinder, cfg: &Config, data: &mut Data, r: &Vec<String>) {
    update_players(pf, cfg, data);
    update_message(cfg, data, r);
    print_text(cfg, data);
}

/// Main function. Mostly concerned with initialisation.
fn main() {
    // Parse cli flags
    let cli = Cli::parse();

    // logging initialisation
    std::env::set_var::<&str, OsString>("RUST_LOG", cli.log_level.into());
    if let Err(e) = env_logger::init() {
        error!("{e}");
        return;
    }

    // Config, Data, and PlayerFinder initialisation
    match confy::load::<Config>("polybar-now-playing", cli.config_file.as_str()) {
        Ok(mut cfg) => {
            if let None = cfg.player_prefixes.get("default") {
                cfg.player_prefixes
                    .insert("default".to_owned(), ">".to_owned());
            }

            let mut data: Data = Data::default();
            let rating_strings = cfg.build_rating_strings();

            let pf: PlayerFinder;
            match PlayerFinder::new() {
                Ok(finder) => pf = finder,
                Err(e) => {
                    error!("{e}");
                    return;
                }
            }

            // signal interception initialisation
            let term = Arc::new(AtomicBool::new(false));
            if let Err(e) =
                signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&term))
            {
                error!("{e}");
                return;
            }

            // main body loop
            loop {
                thread::sleep(time::Duration::from_millis(cfg.update_delay));
                match cli.list {
                    true => print_players(&pf),
                    false => default_loop(&pf, &cfg, &mut data, &rating_strings),
                }

                if term.load(Ordering::Relaxed) {
                    handle_signal(&data);
                    term.swap(false, Ordering::Relaxed);
                };
            }
        }
        Err(e) => {
            error!("{e}");
        }
    };
}
