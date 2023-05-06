use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self};
use mpris::PlayerFinder;
use structs::{config::Config, data::Data};
use crate::update_players::update_players;
use crate::update_message::update_message;
use crate::print_text::print_text;

mod update_players;
mod update_message;
mod print_text;
mod structs;

fn handle_signal(data: &Data, pf: &PlayerFinder) {
  if data.current_player.is_some() {
    if let Ok(p) = pf.find_by_name(data.current_player.as_ref().unwrap()) {
      let _ = p.checked_play_pause();
    }
  }
}

//todo: load different config giles depending on CLI argument
//so i can specify 3 files to use with my polybar config

fn main() {
    //dotenvy::dotenv().expect("Failed to read .env file");
    std::env::set_var("RUST_LOG", "error");
    if let Err(e) = env_logger::init() {
      panic!("{}", e);
    }

    let mut cfg: Config = confy::load("polybar-now-playing", None).unwrap(); //TODO: error handling
    cfg.priorities_to_lower();
    let mut data: Data = Data::default();
    let term = Arc::new(AtomicBool::new(false));

    let pf: PlayerFinder = PlayerFinder::new()
      .expect("Failed to connect to Dbus!");
    
    if let Err(e) = signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&term)) {
      panic!("{}", e);
    }

    loop {
      thread::sleep(cfg.update_delay);
       update_players(&pf, &cfg, &mut data);
       update_message(&pf, &cfg, &mut data);
       print_text(&cfg, &mut data);
       if term.load(Ordering::Relaxed) { 
        handle_signal(&data, &pf);
        term.swap(false, Ordering::Relaxed);
      };
    }
  }
