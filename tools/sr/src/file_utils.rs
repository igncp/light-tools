extern crate clap;
extern crate walkdir;

use walkdir::WalkDir;

pub fn get_all_files(path: String) -> Vec<String> {
  let mut files: Vec<String> = vec![];

  for entry in WalkDir::new(path) {
    match entry {
      Ok(entry) => {
        let metadata = entry.metadata().unwrap();

        if metadata.is_file() {
          files.push(entry.path().display().to_string());
        }
      }
      Err(_) => {}
    };
  }

  files
}
