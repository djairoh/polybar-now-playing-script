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
    let mut active: Vec<Vec<(i32, String)>> = vec![Vec::new(), Vec::new(), Vec::new()];
    for player in players {
      if cfg.player_priorities.contains(&player.identity().to_owned().to_ascii_lowercase()) {
        let name = player.identity();
        let idx = cfg.find_player_priorities_idx(name);
        if let Ok(status) = player.get_playback_status() {
          match status {
            mpris::PlaybackStatus::Playing => active[0].push((idx, name.to_owned())),
            mpris::PlaybackStatus::Paused => active[1].push((idx, name.to_owned())),
            mpris::PlaybackStatus::Stopped => active[2].push((idx, name.to_owned())),
          };
        }
      }
    }
    if !active[0].is_empty() {
      data.current_player =  Some(get_lowest(&active[0]));
    } else if !active[1].is_empty() {
      data.current_player =  Some(get_lowest(&active[1]));
    } else if !active[2].is_empty() {
      data.current_player =  Some(get_lowest(&active[2]));
    } else {
      if let Ok(player) = pf.find_active() {
        data.current_player = Some(player.identity().to_owned());
      } else {
        data.current_player = None;
      }
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