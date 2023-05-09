use log::trace;
use mpris::{PlayerFinder, Metadata, Player};
use crate::structs::{data::Data, config::Config};

fn update_prefix(cfg: &Config, data: &mut char, name: &str) {
  if let Some(char) = cfg.player_prefixes.get(name) {
    *data = char.clone();
    trace!("updated prefix to {}", data);
  } else {
    *data = cfg.player_prefixes.get("default").unwrap_or(&'>').clone();
    trace!("set prefix to default ({})", data);
  }
}

pub fn update_players(
  pf: &PlayerFinder,
  cfg: &Config,
  mut data: &mut Data,
) {
  let players = pf.find_all().unwrap_or(Vec::new());
  if players.is_empty() {
    data.current_player = None;
  } else {
    let mut active: Vec<(i32, Player)> = Vec::new();
    for player in players {
      if let Ok(mpris::PlaybackStatus::Playing) = player.get_playback_status() {
        let idx = cfg.find_player_priorities_idx(player.identity());
        active.push((idx, player));
      }
    }

    if !active.is_empty() {
      let cur = get_lowest(&mut active);
      update_prefix(cfg, &mut data.display_prefix, cur.identity());
      data.current_player = Some(cur);
    } else {
      data.current_player = None;
    }
  }
}

fn get_lowest(v: &mut Vec<(i32, Player)>) -> Player {
  let mut lowest_index = i32::MAX;  //FIXME: use options here instead, also fixes a bug
  for (v_id, _) in v.into_iter() {
    if v_id < &mut lowest_index {
      lowest_index = *v_id;
    }
  }
  v.swap_remove(lowest_index as usize).1
}