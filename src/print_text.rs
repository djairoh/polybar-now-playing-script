use std::collections::HashMap;
use log::info;
use string_builder::Builder;

use crate::structs::{config::{Field, Config}, data::Data};

// TODO: update this function to apply a 'fuzzy' mode instead (possibly add config option).
// This would find the space character nearest the num_chars index and cut off there, allowing for cleaner truncation.
fn cutoff(fields: &Vec<Field>, brk: Option<char>, strings: &mut HashMap<String, String>) {
  for field in fields {
    if !field.field.eq("xesam:userRating") && !field.field.eq("xesam:autoRating") {
      let a = strings.get(&field.field);
      if a.is_some() && a.unwrap().len() >= field.num_chars as usize {
        let mut b = a.unwrap().clone();
        b.truncate(field.num_chars as usize);
        if let Some(c) = brk {
          b.push(c)
        }
        strings.insert(field.field.clone(), b);
      }
    }
  }
}

fn append_prefix(b: &mut Builder, data: &Data) {
  b.append(data.display_prefix.clone());
  b.append("  ");
}

fn append_fields(b: &mut Builder, cfg: &Config, data: &Data) {
  let mut idx = 0; let len = data.display_text.len() as i32;
  for string in &cfg.metadata_fields {
    if let Some(string) = data.display_text.get(&string.field) {
      idx += 1;
      b.append(string.clone());
      if idx < len {b.append(format!(" {} ", cfg.metadata_separator))};
    } else {
      info!("failed to get {} value!", string.field);
    }
  }
}

fn build_string(cfg: &Config, data: &mut Data) -> String {
  let mut b = Builder::default();
 
  if cfg.render_prefix {
    append_prefix(&mut b, data);
  }
  b.append(format!("%{{T{}}}", cfg.font_index));
  append_fields(&mut b, cfg, data);
  b.append("%{T-}");

  b.string().unwrap_or("Failed to unwrap string!".to_owned())
}

pub fn print_text(cfg: &Config, data: &mut Data) {
  if cfg.hide_output && data.current_player.is_none() {
    println!("");
  } else {
    cutoff(&cfg.metadata_fields, cfg.break_character, &mut data.display_text);
    println!("{}", build_string(cfg, data));
  }
}