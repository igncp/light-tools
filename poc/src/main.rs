extern crate walkdir;

use walkdir::WalkDir;

fn walk_directory_and_print_files() {
    let args: Vec<String> = std::env::args().collect();
    let ref path = &args[1];

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                let metadata = entry.metadata().unwrap();

                if metadata.is_file() {
                    println!("{}", entry.path().display());
                }
            }
            Err(_) => {
                println!("Incorrect path");
            }
        };
    }
}

fn main() {
    walk_directory_and_print_files();
}
