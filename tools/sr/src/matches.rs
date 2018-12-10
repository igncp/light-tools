use std::fmt;
use std::fs::read_to_string;

use regex::Regex;

#[derive(Clone)]
pub struct MatchItem {
  pub num: usize,
  pub path: String,
  pub total: usize,
}

impl fmt::Display for MatchItem {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} [num: {}, total: {}]",
      self.path, self.num, self.total
    )
  }
}

pub fn get_matches(all_files: Vec<String>, pattern: String) -> Vec<MatchItem> {
  let mut matches: Vec<MatchItem> = vec![];

  for file in &all_files {
    let contents = read_to_string(file).expect("Something went wrong reading the file");
    let reg: Regex = Regex::new(&pattern).unwrap();
    let mut local_matches: Vec<MatchItem> = vec![];

    for (match_idx, _) in reg.captures_iter(&contents).enumerate() {
      let path = file.clone();
      let match_item = MatchItem {
        num: match_idx + 1,
        total: 0,
        path: path,
      };

      local_matches.push(match_item);
    }

    let total: usize = local_matches.len();

    for local_match in local_matches {
      let final_match = MatchItem {
        path: local_match.path,
        total: total,
        num: local_match.num,
      };

      matches.push(final_match);
    }
  }

  matches
}
