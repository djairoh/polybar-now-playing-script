use std::collections::HashMap;

use mpris::Player;

pub struct Data {
  pub current_player: Option<Player>, 
  pub field_text: HashMap<String, String>,
  pub prefix: char,
}

impl Default for Data {
    fn default() -> Self {
        Self { 
          current_player: None,
          field_text: HashMap::new(), 
          prefix: ' ', 
        }
    }
}