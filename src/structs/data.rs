//! This file contains structs and functions related to data management within the program.
//! It effectively contains the state of the program.
use std::collections::HashMap;

use mpris::Player;

/// This struct concerns itself with the current state of the program.
pub struct Data {
  /// Represents the media player marked as active.
  /// Should be None when no (accepted) players are active.
  pub current_player: Option<Player>, 
  /// HashMap representing the current output strings for each configured field.
  pub field_text: HashMap<String, String>,
  /// What character to render as prefix.
  pub prefix: char,
}

/// Defaults for Data struct.
/// Generates an empty hashmap, and prefix, as well as None for current_player.
impl Default for Data {
    fn default() -> Self {
        Self { 
          current_player: None,
          field_text: HashMap::new(), 
          prefix: ' ', 
        }
    }
}