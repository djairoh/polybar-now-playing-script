//! This file contains structs and functions concerning themselves with the configuration of the program.
use std::{collections::HashMap};
use serde::{Serialize, Deserialize};

/// This struct represents one metadata field to be rendered, as well as the maximum length of its' output.
#[derive(Serialize, Deserialize)]
pub struct Field {
  /// The name of the metadata field.
  pub field: String,
  /// The maximum length of the metadata field's output.
  pub num_chars: u8
}

impl Field {
  /// Create a new field from given values.
  /// input:
  /// field: name of the field
  /// num_chars: maximum length of the field
  /// 
  /// returns:
  /// a new Field with the given parameters.
  fn new(field: String, num_chars: u8) -> Self {
    Field {
      field,
      num_chars
    }
  }

  /// Create a new field from given values.
  /// input:
  /// field: name of the field
  /// num_chars: maximum length of the field
  /// 
  /// returns:
  /// a new Field with the given parameters.
  pub fn constructor(field: &str, num_chars: u8) -> Self {
    Self::new(field.to_owned(), num_chars)
  }
}


/// This struct contains the 3 symbols used to represent a given userRating in a media field.
#[derive(Serialize, Deserialize)]
pub struct Rating {
  /// character for an empty token
  pub nil: char,
  /// character for a half token
  pub half: char,
  /// character for a full token
  pub full: char
}

impl Rating {
  /// This function repeats a given character n times, interspersing each occurence with a space.
  /// It's kinda unwieldy here, but this is the least odd place to put it.
  /// 
  /// input:
  /// c: character to repeat
  /// n: number of times to repeat the character
  /// 
  /// returns:
  /// string of the form '<c> '{n}
  fn repeat(c: char, n: usize) -> String {
    let mut s = c.to_string();
    s.push(' ');
    s.repeat(n)
  }

  /// As there are only a small, run-time defined variances on possible ratings (from 5 empty tokens to 5 full ones),
  /// this function computes all these strings during initialization. This saves a near negligble amount of operations during run-time.
  /// 
  /// output:
  /// Vec of Strings representing all possible rating configurations
  fn build_rating_strings(&self) -> Vec<String> {
    let mut out = Vec::new();
    out.push(Self::repeat(self.nil, 5));
    out.push(format!("{}{}",   Self::repeat(self.half, 1), Self::repeat(self.nil,  4)));
    out.push(format!("{}{}",   Self::repeat(self.full, 1), Self::repeat(self.nil,  4)));
    out.push(format!("{}{}{}", Self::repeat(self.full, 1), Self::repeat(self.half, 1), Self::repeat(self.nil, 3)));
    out.push(format!("{}{}",   Self::repeat(self.full, 2), Self::repeat(self.nil,  3)));
    out.push(format!("{}{}{}", Self::repeat(self.full, 2), Self::repeat(self.half, 1), Self::repeat(self.nil, 2)));
    out.push(format!("{}{}",   Self::repeat(self.full, 3), Self::repeat(self.nil,  2)));
    out.push(format!("{}{}{}", Self::repeat(self.full, 3), Self::repeat(self.half, 1), Self::repeat(self.nil, 1)));
    out.push(format!("{}{}",   Self::repeat(self.full, 4), Self::repeat(self.nil,  1)));
    out.push(format!("{}{}",   Self::repeat(self.full, 4), Self::repeat(self.half, 1)));
    out.push(Self::repeat(self.full, 5));
   out
  }
}

/// Defaults for Rating struct.
/// uses UTF-8, ASCII compatible tokens.
impl Default for Rating {
    fn default() -> Self {
        Self {
          nil: '-',
          half: '/',
          full: '+'
        }
    }
}


/// This struct contains all possible configuration fields. 
/// It should not be used as mutable; all data in this struct should effectively be treated as read-only.
#[derive(Serialize, Deserialize)]
pub struct Config {
  /// Whether to hide the last output if there are currently no accepted players.
  pub hide_output: bool,
  /// Whether to apply 'fuzzy' cutoff to strings exceeding their maximum lenght.
  pub fuzzy: bool,
  /// Whether to render the prefix at all.
  pub render_prefix: bool,
  /// Time in milliseconds to wait between loops of the program.
  pub update_delay: u64,
  /// String to insert between different metadata fields.
  pub metadata_separator: String,
  /// Character to insert between Array values (used when a MetadataVaue is of type Vec (ie multiple artists on one track)).
  pub array_separator: char,
  /// Character to insert when a string is truncated. None implies no cut off character is inserted and the strings are truncated as is.
  pub break_character: Option<char>,
  /// Vec of mpris identities, describing what players are considered acceptable.
  /// Prioritised based on vec index (closer to 0 -> higher priority).
  pub player_priorities: Vec<String>,
  /// Characters to use for the xesam:userRating field. 
  /// If None, default values are used ('-', '/', '+').
  pub rating_icons: Option<Rating>,
  /// Vec of Fields. Each field represents one metadata_string to be shown in output, as well as the maximum number of characters for this field.
  /// Output is shown based on Vec index (vec[0] first, vec[1] second, etc).
  pub metadata_fields: Vec<Field>,
  /// Hashmap which maps Player Identities (strings; key) to prefixes (char; value).
  /// If left blank all players will use the default prefix character ('>').
  pub player_prefixes: HashMap<String, char>,
}

/// Defaults for the Config struct.
/// This is generated when a non-existant config file is specified in the command line.
impl Default for Config {
  fn default() -> Self {
      Config {
        hide_output: true,
        fuzzy: false,
        render_prefix: true,
        update_delay: 300_u64,
        metadata_separator: " | ".to_owned(),
        array_separator: '+',
        break_character: Some('-'),
        player_priorities: Config::default_player_priorities(),
        rating_icons: Some(Rating::default()),
        metadata_fields: Config::default_metadata_fields(),
        player_prefixes: Config::default_player_prefixes(),
      }
  }
}

impl Config {
  /// This function returns the index of a given player identity in the player_priorities hashmap.
  /// If the given identity is not in the map, the value of i32::MAX is returned instead.
  /// 
  /// TODO: using a HashMap would be more efficient i think.
  pub fn find_player_priorities_idx(&self, name: &str) -> i32 {
    match self.player_priorities.iter()
    .position(|x| x.eq(&name)) {
        Some(idx) => idx as i32,
        None => i32::MAX,
    }
  }

  /// This function builds the pre-computed rating strings for a given Rating_icons field.
  pub fn build_rating_strings(&self) -> Vec<String> {
    match self.rating_icons.as_ref() {
        Some(r) => r.build_rating_strings(),
        None => Rating::default().build_rating_strings(),
    }
  }

  /// This function returns the default player_priorities, used when a non-existent config file is requested.
  /// The values of these are based on nothing but my own experience; in fact I'm not even sure if the Spotify app's identity is correct.
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

  /// This function returns the default metadata fields, used when a non-existent config file is requested.
  /// It contains the "title" and "artist" fields, with 40 and 20 maximum characters respectively.
  fn default_metadata_fields() -> Vec<Field> {
    vec![
      Field::constructor("xesam:title", 40),
      Field::constructor("xesam:artist", 20)
    ]
  }

  /// This function returns the default prefixes, used when a non-existent config file is requested.
  /// Like the player priorities function, this is mostly just based on my own experience.
  fn default_player_prefixes() -> HashMap<String, char> {
    let mut out: HashMap<String, char> = HashMap::new();

    out.insert("chromium".to_owned(), 'g');
    out.insert("Clementine".to_owned(), 'c');
    out.insert("default".to_owned(), '>');
    out.insert("Firefox".to_owned(), 'f');
    out.insert("mpv".to_owned(), 'm');
    out.insert("Spotify".to_owned(), 's');
    out.insert("VLC Media Player".to_owned(), 'v');

    out
  }
}

