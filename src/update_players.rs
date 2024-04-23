//! This file deals with updating the active player.
//! It also updates the prefix, which kind of breaks seperation of concerns, but this saves me a lot of headache so I'm not changing it.
use std::collections::BTreeMap;

use crate::structs::{config::Config, data::Data};
use log::{debug, trace};
use mpris::PlayerFinder;

/// This function updates the current prefix.
/// If no entry is found in config containing the active player, a default value is used instead ('>').
///
/// Input:
/// cfg: Config struct for the program, containing the hashmap of prefixes.
/// data: mutable char containing the active prefix.
/// name: name of active player, to fetch the appropriate prefix from cfg.
fn update_prefix(cfg: &Config, data: &mut Data, name: &str) {
    if let Some(char) = cfg.player_prefixes.get(name) {
        data.prefix = char.to_owned();
        trace!("updated prefix to {}", data.prefix);
    } else {
        data.prefix = cfg.player_prefixes.get("default").unwrap().to_owned();
        trace!("set prefix to default ({})", data.prefix);
    }
}

/// This function updates which player is selected as 'active'.
/// It only considers players present in the config.player_priorities field to be valid candidates, then selects the active one with the highest rating.
/// If none of the acceptable players are available, current_player is set to None instead.
///
/// Input:
/// pf: PlayerFinder instance of the program.
/// cfg: Config struct of the program, containing the list of acceptable players.
/// data: mutable Data struct of the program, containing a marker for the currently active player.
pub fn update_players(pf: &PlayerFinder, cfg: &Config, data: &mut Data) {
    // get all acceptable players
    let players = pf.find_all().unwrap_or(Vec::new());
    if players.is_empty() {
        data.current_player = None;
        debug!("update_players: no players found!")
    } else {
        let mut trees = vec![BTreeMap::new(), BTreeMap::new()];
        for player in players {
            if let Ok(status) = player.get_playback_status() {
                let idx = cfg.find_player_priorities_idx(player.identity());
                match status {
                    mpris::PlaybackStatus::Playing => trees[0].insert(idx, player),
                    mpris::PlaybackStatus::Paused => trees[0].insert(idx, player),
                    mpris::PlaybackStatus::Stopped => trees[1].insert(idx, player),
                };
            }
        }

        // select the player with the highest priority.
        for mut tree in trees {
            if let Some((_, player)) = tree.pop_first() {
                update_prefix(cfg, data, player.identity());
                debug!("update_players: updated player to {}!", player.identity());
                data.current_player = Some(player);
                break;
            } else {
                data.current_player = None;
                debug!("update_players: No acceptable player found!");
            }
        }
    }
}
