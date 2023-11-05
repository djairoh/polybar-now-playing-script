//! This file deals with updating the actual message, including proper formatting. 
use log::{debug, trace};
use mpris::MetadataValue;

use crate::structs::{config::Config, data::Data};

/// This function converts a given MetadataValue to a String.
/// Note that two types of the MetadataValue enum are currently unsupported:
/// Both the Map and Unsupported types currently lead the program to panic!
/// The HashMap because I honestly don't know when a metadata value would be encoded as such (and am too lazy to dig through the crate's source code),
/// The Unsupported type should be self-explanatory.
/// 
/// Input:
/// v: MetadataValue to convert.
/// sep: seperation character to insert between entries of a Vec.
/// 
/// Output:
/// String representing the input MetadataValue.
fn value_to_string(v: &MetadataValue, sep: char) -> String {
  match v {
    MetadataValue::String(v) => v.to_string(),
    MetadataValue::I16(v) => v.to_string(),
    MetadataValue::I32(v) => v.to_string(),
    MetadataValue::I64(v) => v.to_string(),
    MetadataValue::U8(v) => v.to_string(),
    MetadataValue::U16(v) => v.to_string(),
    MetadataValue::U32(v) => v.to_string(),
    MetadataValue::U64(v) => v.to_string(),
    MetadataValue::F64(v) => v.to_string(),
    MetadataValue::Bool(v) => v.to_string(),
    MetadataValue::Array(v) => {
      let mut out = v.iter().map( |val| {
        let mut str = value_to_string(val, sep);
        str.push(sep);
        str
      }).collect::<String>();
      out.pop();
      out
    },
    MetadataValue::Map(_v) => panic!("unimplemented! TBH i have no clue when a metadataValue would even return this?"),
    MetadataValue::Unsupported => panic!("Unsupported Metadata type detected!"),
  }
}

/// This function converts one specific instance of MetadataValue to an appropriate String.
/// It deals with the xesam:userRating type. This is a float (0.0 <= v <= 1.0), but should be represented on a scale fron 0 to 10 (according to me).
/// As such, it converts the float value to a visually appealing 5-symbol string.
/// 
/// Input:
/// r: MetadataValue, should be of the enum type f64 (unchecked).
/// str: Vec containing precomputed rating strings to select from.
/// 
/// Output:
/// Some(String) if a rating exists, None otherwise.
fn rating_to_string(r: Option<&MetadataValue>, str: &Vec<String>) -> Option<String> {
  match r {
    Some(rating) => {
      if let Some(f) = rating.as_f64() {
        let mut i = (f * 10_f64).round() as i64;
        if i > 10 {i = 10}
        if i < 0 {i = 0}

        Some(str[i as usize].to_owned()) //TODO: still inefficient. would be better to note the idx and load it in print_text 
      } else {
        debug!("failed to convert MetadataValue to f64!");
        None
      }
    },
    None => {
      trace!("no userRating MetadataValue found!");
      None
    },
  }
}

/// This higher level function updates the to be output Hashmap of strings.
/// It does so by querying each metadata field in config to the current player, then updating the Hashmap in Data with the new value(s).
/// "xesam:userRating" is treated separately, due to requiring a different output format.
/// 
/// Input:
/// cfg: Config struct for the program. Contains the wanted metadata fields.
/// data: mutable Data struct for the program. Its' Hashmap containing strings is updated.
/// ratings: Vec of precomputed rating strings.
pub fn update_message(cfg: &Config, data: &mut Data, ratings: &Vec<String>) {
  if let Some(player) = &data.current_player {
    if let Ok(meta) = player.get_metadata() {
      for field in &cfg.metadata_fields {
        let key: &str = field.field.as_ref();
        if field.field.eq("xesam:userRating") {
          if let Some(rating_string) = rating_to_string(meta.get(key), ratings) {
            data.field_text.insert(key.to_owned(), rating_string);
          } else {
            data.field_text.remove(key);
          }
        } else {
          match meta.get(&key) {
            Some(value) => data.field_text.insert(key.to_owned(), value_to_string(value, cfg.array_separator)),
            None => data.field_text.insert(key.to_owned(), format!("No {}", key.trim_start_matches("xesam:"))),
          };
        }
      }
    }
  }
}