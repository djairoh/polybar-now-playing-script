use std::{collections::HashMap};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Field {
  pub field: String,
  pub num_chars: u8
}

impl Field {
  fn new(metadata_field: String, num_chars: u8) -> Self {
    Field {
      field: metadata_field,
      num_chars
    }
  }
  pub fn constructor(metadata_field: &str, num_chars: u8) -> Self {
    Self::new(metadata_field.to_owned(), num_chars)
  }
}

#[derive(Serialize, Deserialize)]
pub struct Rating {
  pub nil: char,
  pub half: char,
  pub full: char
}

impl Rating {
  pub fn repeat(c: char, n: usize) -> String {
    let mut s = c.to_string();
    s.push(' ');
    s.repeat(n)
  }
}

impl Default for Rating {
    fn default() -> Self {
        Self {
          nil: '-',
          half: '/',
          full: '+'
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub metadata_separator: String,
  pub array_separator: char,
  pub update_delay: u64,
  pub hide_output: bool,
  pub fuzzy: bool,
  pub render_prefix: bool,
  pub break_character: Option<char>,
  pub player_priorities: Vec<String>,
  pub rating_icons: Rating,
  pub metadata_fields: Vec<Field>,
  pub player_prefixes: HashMap<String, char>,
}

impl Default for Config {
  fn default() -> Self {
      Config {
        update_delay: 300_u64,
        metadata_separator: "|".to_owned(),
        array_separator: '+',
        hide_output: true,
        fuzzy: false,
        render_prefix: true,
        metadata_fields: Config::default_metadata_fields(),
        rating_icons: Rating::default(),
        player_priorities: Config::default_player_priorities(),
        break_character: Some('-'),
        player_prefixes: Config::default_player_prefixes(),
      }
  }
}

impl Config {
  pub fn find_player_priorities_idx(&self, name: &str) -> i32 {
    match self.player_priorities.iter()
    .position(|x| x.eq(&name)) {
        Some(idx) => idx as i32,
        None => i32::MAX,
    }
  }

  fn default_player_priorities() -> Vec<String> {
    vec![
      "Clementine".to_owned(),
      "Spotify".to_owned(),
      "mpv".to_owned(),
      "VLC Media Player".to_owned(),
      "Firefox".to_owned(),
      "Chromium".to_owned()
    ]
  }

  fn default_metadata_fields() -> Vec<Field> {
    vec![
      Field::constructor("xesam:title", 40),
      Field::constructor("xesam:artist", 20)
    ]
  }

  fn default_player_prefixes() -> HashMap<String, char> {
    let mut out: HashMap<String, char> = HashMap::new();

    out.insert("Clementine".to_owned(), 'c');
    out.insert("Firefox".to_owned(), 'f');
    out.insert("Spotify".to_owned(), 's');
    out.insert("default".to_owned(), '>');

    out
  }
}

