use std::collections::HashMap;

use mpris::Player;

pub struct Data {
  pub current_player: Option<Player>, 
  pub display_text: HashMap<String, String>,
  pub display_prefix: char,
}

impl Default for Data {
    fn default() -> Self {
        Self { 
          current_player: None,
          display_text: HashMap::new(), 
          display_prefix: ' ', 
        }
    }
}