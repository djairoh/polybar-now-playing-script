use mpris::PlayerFinder;
use crate::structs::{data::Data, config::Config};

pub fn update_players(
  pf: &PlayerFinder,
  cfg: &Config,
  mut data: &mut Data,
) {
  let players = pf.find_all().unwrap_or(Vec::new());
  if players.is_empty() {
    data.current_player = None;
  } else {
    let mut active: Vec<(i32, String)> = Vec::new();
    for player in players {
      if let Ok(mpris::PlaybackStatus::Playing) = player.get_playback_status() {
        let name = player.identity();
        let idx = cfg.find_player_priorities_idx(name);
        active.push((idx, name.to_owned()));
      }
    }

    if !active.is_empty() {
      data.current_player =  Some(get_lowest(&active));
    } else {
      data.current_player = None;
    }
  }
}

fn get_lowest(v: &Vec<(i32, String)>) -> String {
  let mut out = String::new();
  let mut lowest_index = i32::MAX;
  for (v_id, v_str) in v.iter() {
    if v_id < &lowest_index {
      out = v_str.to_owned();
      lowest_index = *v_id;
    }
  }
  out
}