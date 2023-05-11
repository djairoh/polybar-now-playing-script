use mpris::{MetadataValue};

use crate::structs::{config::Config, data::Data};

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

fn rating_to_string(r: Option<&MetadataValue>, str: &Vec<String>) -> Option<String> {
  match r {
    Some(rating) => {
      if let Some(f) = rating.as_f64() {
        let mut i = (f * 10_f64).round() as i64;
        if i > 10 {i = 10}
        if i < 0 {i = 0}

        Some(str[i as usize].clone()) //TODO: still inefficient. would be better to note the idx and load it in print_text 
      } else {
        None
      }
    },
    None => {
      None
    },
  }
}

pub fn update_message(cfg: &Config, data: &mut Data, ratings: &Vec<String>) {
  if let Some(player) = &data.current_player {
    if let Ok(meta) = player.get_metadata() {
      for field in &cfg.metadata_fields {
        let key = field.field.clone();
        if field.field.eq("xesam:userRating") {
          if let Some(rating_string) = rating_to_string(meta.get(&key), ratings) {
            data.field_text.insert(key, rating_string);
          } else {
            data.field_text.remove(&key);
          }
        } else {
          match meta.get(&key) {
            Some(value) => data.field_text.insert(key, value_to_string(value, cfg.array_separator)),
            None => data.field_text.insert(key, format!("No {}", field.field.clone().trim_start_matches("xesam:"))),
          };
        }
      }
    }
  }
}