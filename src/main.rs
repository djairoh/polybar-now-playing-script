use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use clap::Parser;
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
    let _ = p.checked_play_pause();
  }
}

fn default_loop(pf: &PlayerFinder, cfg: &Config, data: &mut Data) {
  update_players(pf, cfg, data);
  update_message(cfg, data);
  print_text(cfg, data);
}

fn main() {
    std::env::set_var("RUST_LOG", "error");
    if let Err(e) = env_logger::init() {
      panic!("{}", e);
    }

    let cli = Cli::parse();
    match confy::load::<Config>("polybar-now-playing", cli.config_file.as_str()) {
      Ok(cfg) => {
        let mut data: Data = Data::default();
        let term = Arc::new(AtomicBool::new(false));
        
        let pf: PlayerFinder = PlayerFinder::new()
          .expect("Failed to connect to Dbus!");
        
        if let Err(e) = signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&term)) {
          panic!("{}", e);
        }

        loop {
          thread::sleep(cfg.update_delay);
          match cli.debug {
            true => print_players(&pf),
            false => default_loop(&pf, &cfg, &mut data),
          }
        
          if term.load(Ordering::Relaxed) {
            handle_signal(&data);
            term.swap(false, Ordering::Relaxed);
          };
        }
      },
    Err(_) => println!("Failed to read config file {}", cli.config_file),
  };
}
