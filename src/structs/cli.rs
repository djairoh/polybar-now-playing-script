use clap::Parser;

/// Program which finds the active mpris player.
/// 
/// Most configuration is done through config files.
#[derive(Parser)]
pub struct Cli {
  /// The name of the config file to use. 
  #[arg(short = 'c', long = "config", default_value = "default")]
  pub config_file: String,
}