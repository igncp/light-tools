#[macro_use]
extern crate serde_derive;

use std::fs::DirBuilder;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg, ArgMatches, SubCommand};
use csv::ReaderBuilder;
use dirs::home_dir;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Record {
  created: String,
  location: String,
  location_id: usize,
  notes: String,
  updated: String,
  what: String,
  what_id: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CSVRecord {
  what: String,
  location: String,
  updated: String,
  notes: String,
}

impl Record {
  fn print_line(&self) {
    print!("- {}", &self.what);
    print!(" | ");
    print!("{}", &self.location);
    print!(" | ");
    print!("{}", &self.updated);
    print!(" | ");
    print!("{}", &self.notes);
    println!();
  }
}

fn get_data_records() -> Vec<Record> {
  let mut file = File::open(".o/o_data");

  if file.is_err() {
    file = File::open([home_dir().unwrap().to_str().unwrap(), "/.o/o_data"].concat());
  }

  let reader = BufReader::new(file.unwrap());
  let records: Vec<Record> = serde_json::from_reader(reader).unwrap();

  records
}

fn init_project() {
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
    .write_all(b"encryption_key = \"change_this\"")
    .unwrap();
}

fn handle_csv(matches: &ArgMatches<'_>) {
  if matches.is_present("import") {
    let file_path = matches.value_of("import").unwrap();

    let mut rdr = ReaderBuilder::new()
      .has_headers(false)
      .from_path(file_path)
      .unwrap();

    let csv_records: Vec<CSVRecord> = rdr
      .records()
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
    let mut records: Vec<Record> = vec![];

    for csv_record in csv_records {
      records.push(Record {
        what: csv_record.what,
        updated: csv_record.updated,
        location: csv_record.location,
        notes: csv_record.notes,
        created: "".to_string(),
        what_id: 0,     // @TODO
        location_id: 0, // @TODO
      });
    }

    let records_json = serde_json::to_string_pretty(&records).unwrap();

    let mut file = OpenOptions::new()
      .write(true)
      .create_new(false)
      .open(".o/o_data")
      .unwrap();
    file.write_all(records_json.as_bytes()).unwrap();
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

fn handle_search(matches: &ArgMatches<'_>) {
  let records = get_data_records();
  let contents = matches.values_of("CONTENT").unwrap().collect::<Vec<&str>>();

  for record in records {
    for content in &contents {
      let content_l = content.to_ascii_lowercase();
      let what_l = record.what.to_ascii_lowercase();
      if what_l.contains(&content_l) {
        record.print_line();
        break;
      }
    }
  }
}

fn parse_args() {
  let matches = App::new("o")
    .version("1.0")
    .about("Organizing helpers")
    .subcommand(SubCommand::with_name("init").about("Inits a new project"))
    .subcommand(
      SubCommand::with_name("csv")
        .about("Imports or exports to CSV format")
        .arg(
          Arg::with_name("import")
            .long("import")
            .short("i")
            .value_name("FILE")
            .help("imports CSV"),
        )
        .arg(
          Arg::with_name("export")
            .long("export")
            .short("e")
            .value_name("FILE")
            .help("exports CSV"),
        ),
    )
    .subcommand(
      SubCommand::with_name("s")
        .about("Search")
        .arg(Arg::with_name("CONTENT").multiple(true)),
    )
    .get_matches();

  if matches.subcommand_matches("init").is_some() {
    init_project();
  } else if let Some(matches) = matches.subcommand_matches("csv") {
    handle_csv(matches);
  } else if let Some(matches) = matches.subcommand_matches("s") {
    handle_search(matches);
  }
}

fn main() {
  parse_args();
}
