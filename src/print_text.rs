use std::collections::HashMap;
use log::info;
use string_builder::Builder;

use crate::structs::{config::{Field, Config}, data::Data};


fn fuzzy_cutoff(str: &str) -> usize{
  str.rfind(char::is_whitespace).unwrap_or(usize::MAX)
}

fn cutoff(fields: &Vec<Field>, brk: Option<char>, fuzzy: bool, strings: &mut HashMap<String, String>) {
  for field in fields {
    if let Some(str) = strings.get_mut(&field.field) {
      if !field.field.eq("xesam:userRating") && str.len() >= field.num_chars as usize {
        str.truncate(field.num_chars as usize);
        if fuzzy {str.truncate(fuzzy_cutoff(str))}
        // The above crashes on non-utf8 characters. FIXME: do something about that
        if let Some(c) = brk {
          str.push(c);
        }
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
  append_fields(&mut b, cfg, data);

  b.string().unwrap_or("Failed to unwrap string!".to_owned())
}

pub fn print_text(cfg: &Config, data: &mut Data) {
  if (cfg.hide_output && data.current_player.is_none()) || data.display_text.is_empty() || cfg.metadata_fields.is_empty() {
    println!("");
  } else {
    cutoff(&cfg.metadata_fields, cfg.break_character, cfg.fuzzy, &mut data.display_text);
    println!("{}", build_string(cfg, data));
  }
}