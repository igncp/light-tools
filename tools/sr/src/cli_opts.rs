extern crate clap;
extern crate walkdir;

use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct CommandOpts {
  pub path: String,
  pub search_replacement: String,
  pub search_pattern: String,
}

pub fn parse_args() -> CommandOpts {
  let matches = App::new("sr")
    .version(VERSION)
    .author("Ignacio Carbajo <icarbajop@gmail.com>")
    .about("Search and replace for the command line")
    .arg(
      Arg::with_name("PATH")
        .help("The path of it")
        .required(true)
        .index(1),
    )
    .arg(
      Arg::with_name("SEARCH_PATTERN")
        .help("What to replace")
        .required(true)
        .index(2),
    )
    .arg(
      Arg::with_name("SEARCH_REPLACEMENT")
        .help("With what to replace it")
        .required(true)
        .index(3),
    )
    .get_matches();

  let path = matches.value_of("PATH").unwrap();
  let search_pattern = matches.value_of("SEARCH_PATTERN").unwrap();
  let search_replacement = matches.value_of("SEARCH_REPLACEMENT").unwrap();

  let opts = CommandOpts {
    path: path.to_string(),
    search_pattern: search_pattern.to_string(),
    search_replacement: search_replacement.to_string(),
  };

  return opts;
}
