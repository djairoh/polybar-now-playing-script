use clap::Parser;

/// Program which finds the active mpris player.
/// 
/// Most configuration is done through config files.
#[derive(Parser)]
pub struct Cli { 
  /// The name of the config file to use. 
  #[arg(short = 'c', long = "config", default_value = "default")]
  pub config_file: String,
  /// Enable debug mod.
  /// 
  /// This mode prints all active players to stdout, to allow one to find the appropriate player names to use in the config files.
  #[arg(short = 'd', long = "debug")]
  pub debug: bool,
}