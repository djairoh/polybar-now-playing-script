//! This file contains functions used in debugging mode.
use log::error;
use mpris::PlayerFinder;


/// This function finds and prints the identities of all players on the system to stdout.
/// It is intended to help people find the right identities to use in their configuration files.
pub fn print_players(pf: &PlayerFinder) {
    match pf.find_all() {
        Ok(players) => {
            if players.is_empty() {
                println!("No players found!");
            } else {
                for player in players {
                    println!("{}", player.identity());
                }
            }
        },
        Err(e) => error!("{e}"),
    }
}