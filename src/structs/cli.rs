//! This file contains structs and functionality that are relevant to the Command Line Interface part of the program.
use clap::Parser;

/// Program which finds the active mpris player and displays metadata about the playing piece of media.
/// 
/// This program is intended to be used with polybar.
/// As such, most configuration is done through config files.
#[derive(Parser)]
pub struct Cli { 
  /// The name of the config file to use. 
  #[arg(short = 'c', long = "config", default_value = "default")]
  pub config_file: String,
  /// Enable debug mode.
  /// 
  /// This mode prints all active players to stdout, to allow one to find the appropriate player names to use in the config files.
  #[arg(short = 'd', long = "debug")]
  pub debug: bool,
}