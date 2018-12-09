extern crate clap;
extern crate walkdir;

mod cli_opts;
mod file_utils;

use cli_opts::parse_args;
use file_utils::get_all_files;

fn main() {
  let opts = parse_args();
  let files = get_all_files(opts.path);

  println!("Files in path: {}", files.len())
}
