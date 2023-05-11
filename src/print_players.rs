use log::error;
use mpris::PlayerFinder;


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