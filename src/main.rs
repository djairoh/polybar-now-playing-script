use core::time;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use clap::Parser;
use log::{error, info};
use mpris::PlayerFinder;
use structs::cli::Cli;
use structs::{config::Config, data::Data};
use crate::update_players::update_players;
use crate::update_message::update_message;
use crate::print_text::print_text;
use crate::print_players::print_players;

mod update_players;
mod update_message;
mod print_text;
mod structs;
mod print_players;

fn handle_signal(data: &Data) {
  if let Some(p) = &data.current_player {
    match p.checked_play_pause() {
        Ok(b) => {
          match b {
            true => info!("Player play/paused succesfully!"),
            false => info!("Failed to send play/pause signal!"),
        }
      },
        Err(e) => error!("{e}"),
    }
  }
}

fn default_loop(pf: &PlayerFinder, cfg: &Config, data: &mut Data, r: &Vec<String>) {
  update_players(pf, cfg, data);
  update_message(cfg, data, r);
  print_text(cfg, data);
}

fn main() {
    std::env::set_var("RUST_LOG", "error");
    if let Err(e) = env_logger::init() {
      error!("{e}");
      return
    }

    let cli = Cli::parse();
    match confy::load::<Config>("polybar-now-playing", cli.config_file.as_str()) {
      Ok(cfg) => {
        let mut data: Data = Data::default();
        let rating_strings = cfg.build_rating_strings();

        let term = Arc::new(AtomicBool::new(false));
        
        let pf: PlayerFinder;        
        match PlayerFinder::new() {
            Ok(finder) => pf = finder,
            Err(e) => {
              error!("{e}");
              return
            },
        }

        if let Err(e) = signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&term)) {
          error!("{e}");
          return
        }

        loop {
          thread::sleep(time::Duration::from_millis(cfg.update_delay));
          match cli.debug {
            true => print_players(&pf),
            false => default_loop(&pf, &cfg, &mut data, &rating_strings),
          }
        
          if term.load(Ordering::Relaxed) {
            handle_signal(&data);
            term.swap(false, Ordering::Relaxed);
          };
        }
      },
    Err(e) => {
      error!("{e}");
    },
  };
}
