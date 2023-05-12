//! This file deals with formatting and outputting to stdout.
use std::collections::HashMap;
use log::{info, error};
use string_builder::Builder;

use crate::structs::{config::{Field, Config}, data::Data};

/// This function finds the last whitespace in a string and returns its' index.
/// If there is no whitespace it returns usize::MAX instead.
fn fuzzy_cutoff(str: &str) -> usize {
  str.rfind(char::is_whitespace).unwrap_or_else( || usize::MAX)
}

/// This function helps deal with non-UTF8 strings.
/// It returns the nearest character boundary from the given usize.
/// Note it only looks before the max_len indicated index (that is, it never returns a value > max_len).
/// 
/// Input:
/// str: string to check
/// max_len: usize to check if it's a boundary or not
/// 
/// Returns:
/// usize indicatheing the index of the character boundary nearest max_len.
fn get_char_boundary(str: &str, max_len: usize) -> usize {
  match max_len > str.len() || str.is_char_boundary(max_len) {
    true => max_len,
    false => {
      let mut idx = max_len;
      while !str.is_char_boundary(idx) {
        idx -= 1;
      }
      idx
    },
  }
}

/// This function applies truncation to each strings in the given hashmap, as dictated by the values in the given Fields.
/// It also applies fuzzy cutoff if the configuration option for this is enabled.
/// 
/// Input:
/// field: Vec of Fields, necessary to know where to truncate each string.
/// brk: Optional, character to insert when a string is truncated.
/// fuzzy: Whether to apply the fuzzy truncation function or not.
/// strings: Hashmap containing the strings to be truncated. Key values should match with the names of the Fields Vec.
fn cutoff(fields: &Vec<Field>, brk: Option<char>, fuzzy: bool, strings: &mut HashMap<String, String>) {
  for field in fields {
    if let Some(str) = strings.get_mut(&field.field) {
      if str.len() >= field.num_chars as usize {
        str.truncate(get_char_boundary(str, field.num_chars as usize));
        if fuzzy {str.truncate(fuzzy_cutoff(str))}
        if let Some(c) = brk {
          str.push(c);
        }
      }
    }
  }
}

/// This function appends the prefix character to the given string builder.
/// 
/// Input:
/// b: mutable String builder to append to.
/// data: Data struct containing the current prefix character.
fn append_prefix(b: &mut Builder, data: &Data) {
  b.append(data.prefix);
  b.append("  ");
}

/// This function appends each field in the Data.field_text Hashmap to the given String builder.
/// It does some formatting as well.
/// 
/// Input:
/// b: mutable String builder to append to.
/// cfg: Config struct for the program.
/// data: Data struct containing the field-text HashMap.
fn append_fields(b: &mut Builder, cfg: &Config, data: &Data) {
  let mut idx = 0; 
  let len = data.field_text.len() as i32;

  for field in &cfg.metadata_fields {
    if let Some(string) = data.field_text.get(&field.field) {
      idx += 1;
      b.append(string.as_str());
      if idx < len {b.append(cfg.metadata_separator.as_str())};
    } else {
      info!("failed to get {} value!", field.field);
    }
  }
}

/// This higher level function formats and appends the entire output to a string builder.
/// 
/// Input:
/// cfg: Config struct for the program.
/// data: Data struct containing the state of the program.
/// 
/// Returns:
/// String to be outputted.
fn build_string(cfg: &Config, data: &Data) -> String {
  let mut b = Builder::default();

  if cfg.render_prefix {
    append_prefix(&mut b, data);
  }
  append_fields(&mut b, cfg, data);

  b.string().unwrap_or_else(|e| {
    error!("{e}");
    "Failed to unwrap string!".to_owned()
  })
}

/// This higher level function calls the appropriate string building function depending on a few settings:
/// If either no metadata is specified in the config or no metadata is currently available => it prints an empty line.
/// If no player is currently active and hide_output is true => it prints an empty line.
/// Else => it builds and prints the appropriate output string.
/// 
/// Input:
/// cfg: Config struct for the program.
/// data: mutable Data struct containing the state of the program.
pub fn print_text(cfg: &Config, data: &mut Data) {
  if (cfg.hide_output && data.current_player.is_none()) || data.field_text.is_empty() || cfg.metadata_fields.is_empty() {
    println!("");
  } else {
    cutoff(&cfg.metadata_fields, cfg.break_character, cfg.fuzzy, &mut data.field_text);
    println!("{}", build_string(cfg, data));
  }
}