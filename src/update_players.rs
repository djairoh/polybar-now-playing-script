use std::collections::BTreeMap;

use log::trace;
use mpris::{PlayerFinder, Player};
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
    let mut active: BTreeMap<i32, Player> = BTreeMap::new();
    for player in players {
      if let Ok(mpris::PlaybackStatus::Playing) = player.get_playback_status() {
        let idx = cfg.find_player_priorities_idx(player.identity());
        active.insert(idx, player);
      }
    }

    if let Some((_, player)) = active.pop_first() {
      update_prefix(cfg, &mut data.display_prefix, player.identity());
      data.current_player = Some(player);
    } else {
      data.current_player = None;
    }
  }
}