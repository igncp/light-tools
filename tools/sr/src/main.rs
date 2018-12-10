extern crate clap;
extern crate ncurses;
extern crate regex;
extern crate walkdir;

mod cli_opts;
mod file_utils;
mod matches;
mod ui_containers;

use cli_opts::parse_args;
use file_utils::get_all_files;
use matches::get_matches;
use ui_containers::init_matches_ui;

fn main() {
  let opts = parse_args();
  let all_files = get_all_files(opts.path);

  let matched_items = get_matches(all_files, opts.search_pattern);

  init_matches_ui(matched_items);
}
