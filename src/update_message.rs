use log::{debug, trace};
use mpris::{PlayerFinder, MetadataValue};

use crate::structs::{config::{Rating, Config}, data::Data};


fn update_prefix(cfg: &Config, data: &mut Data) {
  if data.current_player.is_some() {
    let c = cfg.player_prefixes.get(&data.current_player.as_ref().unwrap().to_ascii_lowercase());
    if let Some(char) = c {
      data.display_prefix = char.clone();
      trace!("updated prefix to {}", data.display_prefix);
    } else {
      data.display_prefix = cfg.player_prefixes.get("default").unwrap().clone();  //todo: error handling
      trace!("set prefix to default ({})", data.display_prefix);
    }
  }
}

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
      let f = (rating.as_f64().unwrap() * 10_f64).round() as i64;
      match f { //todo: refactor
        0 => Some(Rating::repeat(map.nil, 5)),
        1 => Some(format!("{}{}", Rating::repeat(map.half, 1), Rating::repeat(map.nil, 4))),
        2 => Some(format!("{}{}", Rating::repeat(map.full, 1), Rating::repeat(map.nil, 4))),
        3 => Some(format!("{}{}{}", Rating::repeat(map.full, 1), Rating::repeat(map.half, 1), Rating::repeat(map.nil, 3))),
        4 => Some(format!("{}{}", Rating::repeat(map.full, 2), Rating::repeat(map.nil, 3))),
        5 => Some(format!("{}{}{}", Rating::repeat(map.full, 2), Rating::repeat(map.half, 1), Rating::repeat(map.nil, 2))),
        6 => Some(format!("{}{}", Rating::repeat(map.full, 3), Rating::repeat(map.nil, 2))),
        7 => Some(format!("{}{}{}", Rating::repeat(map.full, 3), Rating::repeat(map.half, 1), Rating::repeat(map.nil, 1))),
        8 => Some(format!("{}{}", Rating::repeat(map.full, 4), Rating::repeat(map.nil, 1))),
        9 => Some(format!("{}{}", Rating::repeat(map.full, 4), Rating::repeat(map.half, 1))),
        10.. => Some(Rating::repeat(map.full, 5)),
        _ =>  Some(format!("Invalid rating!"))
      }
    },
    None => {
      None
      // Rating::repeat(map.nil, 5)
    },
  }
}

pub fn update_message(pf: &PlayerFinder, cfg: &Config, data: &mut Data) {
  if data.current_player.is_some() {
    update_prefix(cfg, data);
    let name = &data.current_player.as_ref().unwrap();
    if let Ok(player) = pf.find_by_name(name) {
      debug!("found player!");
      if let Ok(m) = player.get_metadata() {
        debug!("got metadata!");
        for field in &cfg.metadata_fields {
          if field.field.eq("xesam:userRating") || field.field.eq("xesam:autoRating") {
            let key = field.field.clone();
            let string = rating_to_string(m.get(&key), &cfg.rating_icons);
            if let Some(str) = string {
              data.display_text.insert(field.field.clone(), str);
            } else {
              data.display_text.remove(&key);
            }
          } else {
            let key = field.field.clone();
            match m.get(&key) {
              Some(value) => {
                debug!("inserting {}: '{}'", key, value_to_string(value, cfg.array_separator));
                data.display_text.insert(key, value_to_string(value, cfg.array_separator));
              },
              None => {
                debug!("field {} is empty!", key);
                data.display_text.insert(key, format!("No {}", field.field.clone().trim_start_matches("xesam:")));              
              },
            }
          }
        }
      }
    }
  }
}