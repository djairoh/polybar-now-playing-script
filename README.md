<p align="center">
  A highly configurable showcaser for active media players.
</p>

**[Installation](#installation) | [Configuration](#configuration)**

This script allows you to showcase information about the active mpris media player in any <a href="github.com/polybar/polybar">Polybar</a> configuration.

![default configuration screenshot](doc/_static/default.png)

## Table of Contents

* [Introduction](#introduction)
* [Installation](#installation)
* [Customization](#customization)
* [Defaults](#defaults)
* [Documentation](#documentation)
* [License](#license)

## Introduction

This script allows one to show the currently actoive media player. It has a variety of configuration options - handled through one or more config files - that allow one to customize the output to their liking. The script makes use of MPRIS media player identities, and only accepts these as valid players.

<!-- The main purpose of **Polybar** is to help users create awesome status bars.
It has built-in functionality to display information about the most commonly used services.
Some of the services included so far:

- Systray icons
- Window title
- Playback controls and status display for [MPD](https://www.musicpd.org/) using [libmpdclient](https://www.musicpd.org/libs/libmpdclient/)
- [ALSA](https://www.alsa-project.org/main/index.php/Main_Page) and [PulseAudio](https://www.freedesktop.org/wiki/Software/PulseAudio/) volume controls
- Workspace and desktop panel for [bspwm](https://github.com/baskerville/bspwm) and [i3](https://github.com/i3/i3)
- Workspace module for [EWMH compliant](https://specifications.freedesktop.org/wm-spec/wm-spec-1.3.html#idm140130320786080) window managers
- Keyboard layout and indicator status
- CPU and memory load indicator
- Battery display
- Network connection details
- Backlight level
- Date and time label
- Time-based shell script execution
- Command output tailing
- User-defined menu tree
- Inter-process messaging
- And more...

[See the wiki for more details](https://github.com/polybar/polybar/wiki). -->

## Installation

Installing the script is straightforward; simply download the appropriate executable from the releases tab, and place it somewhere easily remembered - personally I like to keep all my polybar scripts in ~/.config/polybar/scripts.

To use the script, use the following module in your polybar config:
```
[module/now-playing]
type = custom/script
tail = true
format = <label>
exec = ~/.config/polybar/scripts/polybar-now-playing-rust
click-left = "kill -USR1 $(pgrep --oldest --parent %pid%)"
click-right = "kill -USR1 $(pgrep --oldest --parent %pid%)"
```

In this case, either left- or right-clicking on the script will pause the displayed media player, but these lines are optional and may be left out.

### Building from source

Thanks to Rust, building from source is easy as well. To do so, simply download this repo and then run "cargo build --release"

```
git clone github.com/djairoh/polybar-now-playing-script
cd polybar-now-playing
cargo build --release
```

When the build process finishes, the executable file may be found at target/release/polybar-now-playing-rust[.exe]


## Customization
The main strength of this project lies in its' veritable customization. There are a few flags available when running the script, but most of the customization is reached through use of configuration files.

### CLI arguments
When calling the script with the flag `--help`, the following is printed:
```
Program which finds the active mpris player and displays metadata about the playing piece of media.

This program is intended to be used with polybar. As such, most configuration is done through config files.

Usage: polybar-now-playing-rust [OPTIONS]

Options:
  -c, --config <CONFIG_FILE>
          The name of the config file to use

          [default: default]

  -l, --list
          Enable list mode.

          This mode prints all active players to stdout, to allow one to find the appropriate player names to use in the config files.

      --log <LOG_LEVEL>
          Set log level.

          Sets the log level to print to stdout.

          [default: error]
          [possible values: trace, debug, info, warn, error]

  -h, --help
          Print help (see a summary with '-h')
```

### Config files
Much more interesting, of course, are the various options available in the configuration files. Below is detailed a full example of a config file, complete with annotations explaining each value.

<details>
  <summary>All available options and succint explanations</summary>

  ```toml
# hides the last output if there is currently no active player
# boolean
hide_output = true
# applies an extra function to fields whose output exceeds num_chars. The function will try to truncate the string at the nearest whitespace character before num_chars
# boolean
fuzzy = false
# whether to display the prefix characters in the output string at all
# boolean
render_prefix = true

# time taken between updates of the output string, in milliseconds
# u64 (0 <= u64 <= 18446744073709551615)
update_delay = 300

# what string to use between metadata fields
# string
metadata_separator = ' | '
# what character to use to separate values in an array (ie mediafile with multiple artist metadata entries)
# char
array_separator = '+'
# what character to insert when a field is truncated
# char; optional
break_character = '-'


# What mpris identities to consider for output. Players not in this map will never be used. Values closer to 0 are considered higher priority.
# Note that each string should be paired with an unique u8; if this is not the case, one of the duplicates will be chosen at random during startup and all others discarded (undefined behaviour).
# Similarly, each key should occur at most once.
# HashMap<String, u8> where u8: 0 <= u8 <= 255
[player_priorities]
Clementine = 1
Spotify = 2
mpv = 3
"VLC Media Player" = 4
Firefox = 5
Chromium = 6


# what icons to use for the xesam:userRating field.
# if left blank and the xesam:userRating field is enabled, default values will be used.
# char, char, char; optional
[rating_icons]
nil = '-'
half = '/'
full = '+'


# The following represent metadata_fields to include in the output. To add new entries, use the following format:
#   [[metadata_fields]]
#   field = '<name of field>'
#   num_chars = <maximum number of characters>
# See https://www.freedesktop.org/wiki/Specifications/mpris-spec/metadata/ for available fields.
# string, u8 (0 <= u8 <= 255)
[[metadata_fields]]
field = 'xesam:title'
num_chars = 40

[[metadata_fields]]
field = 'xesam:artist'
num_chars = 20


# The prefixes to use with various players. Each entry is keyed by the Mpris identity.
# This map should contain an entry with the "default" key - although one is hard-coded to be used if it is absent. Leaving the map empty results in all players being rendered with the hard-coded default value ('>').
# If you don't want the program to use prefixes at all, set the render_prefix option earlier in this config to 'false'.
# HashMap<String, char>
[player_prefixes]
chromium = 'g'
Clementine = 'c'
default = '>'
Firefox = 'f'
mpv = 'm'
Spotify = 's'
"VLC Media Player" = 'v'
  ```
</details>

## Defaults
Whenever a config file is specified that does not actually exist, the script creates a new file and populates it with some default values. For posterity, these defaults are included below.

<details>
  <summary>Default configuration file</summary>

  ```toml
  hide_output = true
  fuzzy = false
  render_prefix = true
  update_delay = 300
  metadata_separator = ' | '
  array_separator = '+'
  break_character = '-'

  [player_priorities]
  mpv = 3
  Firefox = 5
  Chromium = 6
  Clementine = 1
  Spotify = 2
  "VLC Media Player" = 4

  [rating_icons]
  nil = '-'
  half = '/'
  full = '+'

  [[metadata_fields]]
  field = 'xesam:title'
  num_chars = 40

  [[metadata_fields]]
  field = 'xesam:artist'
  num_chars = 20

  [player_prefixes]
  Firefox = 'f'
  mpv = 'm'
  Spotify = 's'
  chromium = 'g'
  Clementine = 'c'
  default = '>'
  "VLC Media Player" = 'v'
  ```
</details>


## Documentation
All code in this repository is documented using Rustdoc. The documentation can be compiled and viewed by entering the command `cargo doc --open` while in the base directory.

## License
This script is licensed under the GPL-3.0 license. [see LICENSE for further explanation](https://github.com/djairoh/polybar-now-playing-script/blob/main/LICENSE)