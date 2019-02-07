use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{copy, read_dir, remove_file, rename, DirBuilder, File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;

use clap::ArgMatches;
use csv::ReaderBuilder;
use dirs::home_dir;

use crate::data::{Config, Record};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CSVRecord {
  what: String,
  location: String,
  updated: String,
  notes: String,
}

fn get_data_records() -> Vec<Record> {
  let mut file = File::open(".o/o_data");

  if file.is_err() {
    file = File::open([home_dir().unwrap().to_str().unwrap(), "/.o/o_data"].concat());
  }

  let reader = BufReader::new(file.unwrap());
  let records: Vec<Record> = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);

  records
}

pub fn init_project() {
  let project_dir = ".o";

  DirBuilder::new()
    .recursive(true)
    .create(project_dir)
    .unwrap();

  let mut git_ignore_file = File::create([project_dir, "/.gitignore"].concat()).unwrap();
  let mut config_file = File::create([project_dir, "/o_config.toml"].concat()).unwrap();

  File::create([project_dir, "/o_data"].concat()).unwrap();

  git_ignore_file.write_all(b"o_config.toml").unwrap();
  config_file
    .write_all(
      b"encryption_key = \"change_this\"
saved_actions = 10",
    )
    .unwrap();

  let backups_dir_path = [project_dir, "/backups"].concat();

  DirBuilder::new()
    .recursive(true)
    .create(backups_dir_path)
    .unwrap();
}

fn get_path_exists(path: &str) -> bool {
  std::path::Path::new(path).exists()
}

fn get_config_dir() -> Option<String> {
  let home_dir_path = [home_dir().unwrap().to_str().unwrap(), "/.o"].concat();
  let local_dir_path = ".o";

  if get_path_exists(local_dir_path) {
    return Some(local_dir_path.to_string());
  }

  if get_path_exists(&home_dir_path) {
    return Some(home_dir_path);
  }

  None
}

fn get_file_to_save_data() -> std::fs::File {
  let maybe_dir_path = get_config_dir();

  if maybe_dir_path.is_none() {
    println!("There is no .o directory present locally or in the home directory");
    std::process::exit(1);
  }

  let dir_path = maybe_dir_path.unwrap();

  let saved_actions = get_config().saved_actions;
  let data_path = [&dir_path, "/o_data"].concat();
  let backups_dir_path = [&dir_path, "/backups"].concat();

  let paths: Vec<String> = read_dir(&backups_dir_path)
    .unwrap()
    .map(|x| x.unwrap().path().display().to_string())
    .collect();

  if saved_actions > 0 && paths.len() == saved_actions {
    let backup_file_path = [
      &backups_dir_path,
      "/o_data_prev_",
      saved_actions.to_string().as_str(),
    ]
    .concat();

    remove_file(&backup_file_path).ok();
  }

  for n in 1..saved_actions {
    let i = saved_actions - n;
    let orig_backup_file_path =
      [&backups_dir_path, "/o_data_prev_", i.to_string().as_str()].concat();
    let next_backup_file_path = [
      &backups_dir_path,
      "/o_data_prev_",
      (i + 1).to_string().as_str(),
    ]
    .concat();

    rename(&orig_backup_file_path, &next_backup_file_path).ok();
  }

  if saved_actions > 0 {
    let backup_file_path = [&backups_dir_path, "/o_data_prev_1"].concat();

    copy(&data_path, &backup_file_path).ok();
  }

  remove_file(&data_path).ok();

  OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(&data_path)
    .unwrap()
}

pub fn write_all_records(records: &[Record]) {
  let mut file = get_file_to_save_data();

  let records_json = serde_json::to_string_pretty(&records).unwrap();
  file.write_all(records_json.as_bytes()).unwrap();
}

pub fn get_config() -> Config {
  let mut file = File::open(".o/o_config.toml");

  if file.is_err() {
    let home_dir_config_path =
      [home_dir().unwrap().to_str().unwrap(), "/.o/o_config.toml"].concat();
    file = File::open(&home_dir_config_path);
  }

  let mut contents = String::new();
  file
    .unwrap()
    .read_to_string(&mut contents)
    .expect("Unable to read the file");

  toml::from_str(&contents).unwrap()
}

pub fn handle_csv(matches: &ArgMatches<'_>) {
  if matches.is_present("import") {
    let file_path = matches.value_of("import").unwrap();

    let mut rdr = ReaderBuilder::new()
      .has_headers(false)
      .from_path(file_path)
      .unwrap();

    let csv_records: Vec<CSVRecord> = rdr
      .records()
      .skip(1)
      .map(|x| {
        let result = x.unwrap();
        CSVRecord {
          what: result[0].to_string(),
          location: result[1].to_string(),
          updated: result[2].to_string(),
          notes: result[3].to_string(),
        }
      })
      .collect();
    let csv_records_len = csv_records.len();
    let mut records: Vec<Record> = vec![];

    let mut what_ids: HashMap<String, usize> = HashMap::new();
    let mut location_ids: HashMap<String, usize> = HashMap::new();

    for (idx, csv_record) in csv_records.iter().enumerate() {
      let what = (*csv_record.what).to_string();
      let location = (*csv_record).location.to_string();
      let what_id = idx;
      let location_id = csv_records_len + idx;

      records.push(Record {
        what: what.clone(),
        updated: (*csv_record).updated.to_string(),
        location: location.clone(),
        notes: (*csv_record).notes.to_string(),
        created: (*csv_record).updated.to_string(),
        what_id,
        location_id: csv_records_len + idx,
      });

      if what_ids.get(&what).is_some() {
        println!("Duplicated 'what' entry: {}", what);
        std::process::exit(1);
      } else if location_ids.get(&location).is_none() {
        location_ids.insert(location, location_id);
      }

      what_ids.insert(what, what_id);
    }

    for (idx, record) in records.clone().iter().enumerate() {
      if what_ids.get(&record.location).is_some() {
        records[idx].location_id = what_ids[&record.location];
      } else if location_ids.get(&record.location).is_some() {
        records[idx].location_id = location_ids[&record.location];
      }
    }

    optimize_records_ids(&mut records);

    write_all_records(&records);
  } else if matches.is_present("export") {
    let file_path = matches.value_of("export").unwrap();
    let records = get_data_records();
    let mut csv_records: Vec<CSVRecord> = vec![];

    for record in records.clone() {
      csv_records.push(CSVRecord {
        what: record.what,
        location: record.location,
        updated: record.updated,
        notes: record.notes,
      });
    }

    let mut wtr = csv::Writer::from_path(file_path).unwrap();
    for record in records {
      wtr.serialize(&record).unwrap();
    }
    wtr.flush().unwrap();
  }
}

pub fn optimize_records_ids(records: &mut Vec<Record>) {
  fn get_last_correct_id_idx(existing_ids: &[usize], starting_item: usize) -> Option<usize> {
    let offset = starting_item + 1;
    for (idx, existing_id) in existing_ids.iter().skip(offset).enumerate() {
      let real_idx = idx + offset;

      if *existing_id != real_idx {
        return Some(offset - 1);
      }
    }

    None
  }

  let mut existing_ids_set: HashSet<usize> = HashSet::new();

  for record in records.iter() {
    existing_ids_set.insert(record.what_id);
    existing_ids_set.insert(record.location_id);
  }

  let mut existing_ids = existing_ids_set.iter().cloned().collect::<Vec<usize>>();
  existing_ids.sort();

  if existing_ids[0] != 0 {
    let next_wrong_id = existing_ids[0];
    let next_correct_id = 0;

    for record in records.iter_mut() {
      if record.what_id == next_wrong_id {
        record.what_id = next_correct_id;
      } else if record.location_id == next_wrong_id {
        record.location_id = next_correct_id;
      }
    }
  }

  let mut last_correct_id_idx = get_last_correct_id_idx(&existing_ids, 0);

  while last_correct_id_idx != None {
    let last_correct_id_idx_val = last_correct_id_idx.unwrap();
    let last_correct_id = existing_ids[last_correct_id_idx_val];
    let next_correct_id = last_correct_id + 1;
    let next_wrong_id = existing_ids[last_correct_id_idx_val + 1];

    for record in records.iter_mut() {
      if record.what_id == next_wrong_id {
        record.what_id = next_correct_id;
      } else if record.location_id == next_wrong_id {
        record.location_id = next_correct_id;
      }
    }

    existing_ids[last_correct_id_idx_val + 1] = next_correct_id;

    last_correct_id_idx = get_last_correct_id_idx(&existing_ids, last_correct_id_idx_val + 1);
  }
}

pub fn revert_data_to_backup() {
  let maybe_dir_path = get_config_dir();

  if maybe_dir_path.is_none() {
    println!("There is no .o directory present locally or in the home directory");
    std::process::exit(1);
  }

  let dir_path = maybe_dir_path.unwrap();

  let saved_actions = get_config().saved_actions;

  if saved_actions == 0 {
    println!("Using backup is disabled: `saved_actions` in `.o/o_config.toml`");
    std::process::exit(1);
  }

  let data_path = [&dir_path, "/o_data"].concat();
  let backups_dir_path = [&dir_path, "/backups"].concat();

  let paths: Vec<String> = read_dir(&backups_dir_path)
    .unwrap()
    .map(|x| x.unwrap().path().display().to_string())
    .collect();

  if paths.is_empty() {
    println!("No backups remaining");
    std::process::exit(1);
  }

  let first_backup_file = [&backups_dir_path, "/o_data_prev_1"].concat();

  remove_file(&data_path).ok();
  rename(&first_backup_file, &data_path).ok();

  for n in 2..=paths.len() {
    let orig_backup_file_path =
      [&backups_dir_path, "/o_data_prev_", n.to_string().as_str()].concat();
    let next_backup_file_path = [
      &backups_dir_path,
      "/o_data_prev_",
      (n - 1).to_string().as_str(),
    ]
    .concat();

    rename(&orig_backup_file_path, &next_backup_file_path).ok();
  }

  println!("One write action was reverted");
}
