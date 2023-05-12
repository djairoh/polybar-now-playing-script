//! This file contains structs and functionality that are relevant to the Command Line Interface part of the program.
use std::ffi::OsString;

use clap::Parser;

/// Custom enum to define the desired loglevel during run-time.
#[derive(clap::ValueEnum, Clone)]
pub enum LogLevel {
  TRACE,
  DEBUG,
  INFO,
  WARN,
  ERROR,
}

/// Implement Into<OsString> for LogLevel, so it can actually be used by env_logger.
impl Into<OsString> for LogLevel {
  fn into(self) -> OsString {
    match self {
      LogLevel::TRACE => "trace".into(),
      LogLevel::DEBUG => "debug".into(),
      LogLevel::INFO => "info".into(),
      LogLevel::WARN => "warn".into(),
      LogLevel::ERROR => "error".into(),
    }
  }
}

/// Program which finds the active mpris player and displays metadata about the playing piece of media.
/// 
/// This program is intended to be used with polybar.
/// As such, most configuration is done through config files.
#[derive(Parser)]
pub struct Cli { 
  /// The name of the config file to use. 
  #[arg(short = 'c', long = "config", default_value = "default")]
  pub config_file: String,
  /// Enable list mode.
  /// 
  /// This mode prints all active players to stdout, to allow one to find the appropriate player names to use in the config files.
  #[arg(short = 'l', long = "list")]
  pub list: bool,
  /// Set log level.
  /// 
  /// Sets the log level to print to stdout.
  #[arg(long = "log", value_enum, default_value = "error")]
  pub log_level: LogLevel
}