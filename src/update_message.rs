use mpris::{MetadataValue};

use crate::structs::{config::{Rating, Config}, data::Data};

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

fn rating_to_string(r: Option<&MetadataValue>, map: &Rating) -> Option<String> {
  match r {
    Some(rating) => {
      if let Some(f) = rating.as_f64() {
        let i = (f * 10_f64.round()) as i64;
        match i {
          0 => Some(Rating::repeat(map.nil, 5)),
          1 => Some(format!("{}{}",   Rating::repeat(map.half, 1), Rating::repeat(map.nil,  4))),
          2 => Some(format!("{}{}",   Rating::repeat(map.full, 1), Rating::repeat(map.nil,  4))),
          3 => Some(format!("{}{}{}", Rating::repeat(map.full, 1), Rating::repeat(map.half, 1), Rating::repeat(map.nil, 3))),
          4 => Some(format!("{}{}",   Rating::repeat(map.full, 2), Rating::repeat(map.nil,  3))),
          5 => Some(format!("{}{}{}", Rating::repeat(map.full, 2), Rating::repeat(map.half, 1), Rating::repeat(map.nil, 2))),
          6 => Some(format!("{}{}",   Rating::repeat(map.full, 3), Rating::repeat(map.nil,  2))),
          7 => Some(format!("{}{}{}", Rating::repeat(map.full, 3), Rating::repeat(map.half, 1), Rating::repeat(map.nil, 1))),
          8 => Some(format!("{}{}",   Rating::repeat(map.full, 4), Rating::repeat(map.nil,  1))),
          9 => Some(format!("{}{}",   Rating::repeat(map.full, 4), Rating::repeat(map.half, 1))),
          10.. => Some(Rating::repeat(map.full, 5)),
          _ =>  Some(format!("Invalid rating!"))
        }
      } else {
        None
      }
    },
    None => {
      None
    },
  }
}

pub fn update_message(cfg: &Config, data: &mut Data) {
  if let Some(player) = &data.current_player {
    if let Ok(meta) = player.get_metadata() {
      for field in &cfg.metadata_fields {
        let key = field.field.clone();
        if field.field.eq("xesam:userRating") {
          if let Some(rating_string) = rating_to_string(meta.get(&key), &cfg.rating_icons) {
            data.display_text.insert(key, rating_string);
          } else {
            data.display_text.remove(&key);
          }
        } else {
          match meta.get(&key) {
            Some(value) => data.display_text.insert(key, value_to_string(value, cfg.array_separator)),
            None => data.display_text.insert(key, format!("No {}", field.field.clone().trim_start_matches("xesam:"))),
          };
        }
      }
    }
  }
}