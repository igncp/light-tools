#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::remove_file;
use std::fs::DirBuilder;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

use chrono::{DateTime, Local};
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
    print!(" [{}]", &self.what_id);
    print!(" | ");
    print!("{}", &self.location);
    print!(" [{}]", &self.location_id);
    print!(" | ");
    print!("{}", &self.updated);
    print!(" | ");
    print!("{}", &self.notes);
    println!();
  }

  fn print_location_with_count(&self, context: &Context) {
    let count = context.hierarchy[&self.location_id].children.len();

    print!("- {}", &self.location);
    print!(" [{}]", &self.location_id);
    print!(" <{} items>", count);
    println!();
  }
}

fn get_now_date() -> String {
  let now: DateTime<Local> = Local::now();

  now.format("%d/%m/%y").to_string()
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

fn get_is_empty_text(txt: &str) -> bool {
  txt == "" || txt == "_"
}

fn get_empty_notes_text() -> String {
  "N/A".to_string()
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

fn write_all_records(records: &[Record]) {
  let records_json = serde_json::to_string_pretty(&records).unwrap();

  let mut file = OpenOptions::new()
    .write(true)
    .create_new(false)
    .open(".o/o_data");

  if file.is_err() {
    remove_file([home_dir().unwrap().to_str().unwrap(), "/.o/o_data"].concat()).ok();

    file = OpenOptions::new()
      .write(true)
      .create_new(true)
      .open([home_dir().unwrap().to_str().unwrap(), "/.o/o_data"].concat());
  } else {
    remove_file(".o/o_data").unwrap();
  }

  let mut file = file.unwrap();

  file.write_all(records_json.as_bytes()).unwrap();
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
        panic!("Duplicated 'what' entry: {}", what);
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

#[derive(Debug, Clone)]
struct TreeNode {
  children: HashSet<usize>,
  parent: Option<usize>,
}

#[derive(Debug, Clone)]
struct Context {
  id_to_str_map: HashMap<usize, String>,
  str_to_id_map: HashMap<String, usize>,
  id_to_record_idx_map: HashMap<usize, usize>,
  hierarchy: HashMap<usize, TreeNode>,
  max_id: usize,
}

fn get_context(records: &[Record]) -> Context {
  let mut str_to_id_map: HashMap<String, usize> = HashMap::new();
  let mut id_to_str_map: HashMap<usize, String> = HashMap::new();
  let mut id_to_record_idx_map: HashMap<usize, usize> = HashMap::new();
  let mut hierarchy: HashMap<usize, TreeNode> = HashMap::new();
  let mut max_id = 0;

  for (idx, record) in records.iter().enumerate() {
    let empty_branch = TreeNode {
      parent: None,
      children: HashSet::new(),
    };

    str_to_id_map.insert(record.what.clone(), record.what_id);
    str_to_id_map.insert(record.location.clone(), record.location_id);

    id_to_str_map.insert(record.what_id, record.what.clone());
    id_to_str_map.insert(record.location_id, record.location.clone());

    let what_branch = hierarchy
      .entry(record.what_id)
      .or_insert_with(|| empty_branch.clone());
    what_branch.parent = Some(record.location_id);

    let location_branch = hierarchy
      .entry(record.location_id)
      .or_insert_with(|| empty_branch.clone());
    location_branch.children.insert(record.what_id);

    max_id = std::cmp::max(max_id, record.location_id);
    max_id = std::cmp::max(max_id, record.what_id);

    id_to_record_idx_map.insert(record.what_id, idx);
  }

  Context {
    str_to_id_map,
    id_to_str_map,
    id_to_record_idx_map,
    hierarchy,
    max_id,
  }
}

fn optimize_records_ids(records: &mut Vec<Record>) {
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

fn handle_search(matches: &ArgMatches<'_>) {
  let records = get_data_records();
  let contents = matches.values_of("CONTENT").unwrap().collect::<Vec<&str>>();
  let skip_location = matches.is_present("skip-location");
  let skip_what = matches.is_present("skip-what");

  for record in records {
    let what_l = record.what.to_ascii_lowercase();
    let location_l = record.location.to_ascii_lowercase();

    for content in &contents {
      let parsed = content.parse::<usize>();
      if parsed.is_ok() {
        let id = parsed.unwrap();

        if !skip_what && record.what_id == id || !skip_location && record.location_id == id {
          record.print_line();
          break;
        }
      }

      let content_l = content.to_ascii_lowercase();
      if !skip_what && what_l.contains(&content_l)
        || !skip_location && location_l.contains(&content_l)
      {
        record.print_line();
        break;
      }
    }
  }
}

fn get_full_contents(contents: &[&str]) -> Vec<String> {
  let mut full_contents: Vec<String> = vec![contents[0].to_string()];

  for content in contents.iter().skip(1) {
    if *content != "$" {
      let last = full_contents.pop().unwrap();
      let full_last = [last, content.to_string()].join(" ");
      full_contents.push(full_last.trim().to_string());
    } else {
      full_contents.push("".to_string());
    }
  }

  full_contents
}

fn handle_insert(matches: &ArgMatches<'_>) {
  let contents = matches.values_of("CONTENT").unwrap().collect::<Vec<&str>>();
  let full_contents: Vec<String> = get_full_contents(&contents);

  let mut records = get_data_records();
  let notes = if full_contents.len() > 2 {
    full_contents[2].to_string()
  } else {
    get_empty_notes_text()
  };

  let context = get_context(&records);

  let mut what = full_contents[0].to_string();
  let mut what_id = context.max_id + 1;

  if what.parse::<usize>().is_ok() {
    what_id = what.parse::<usize>().unwrap();
    what = context.id_to_str_map[&what_id].clone();
  } else if context.str_to_id_map.get(&what).is_some() {
    what_id = context.str_to_id_map[&what];
  }

  let mut location = full_contents[1].to_string();
  let mut location_id = context.max_id + 2;

  if location.parse::<usize>().is_ok() {
    location_id = location.parse::<usize>().unwrap();
    location = context.id_to_str_map[&location_id].clone();
  } else if context.str_to_id_map.get(&location).is_some() {
    location_id = context.str_to_id_map[&location];
  }

  for record in &records {
    if record.what == what {
      panic!("Duplicated what: {}", what);
    }
  }

  let created = get_now_date();
  let updated = created.clone();
  let new_record: Record = Record {
    what,
    what_id,
    location,
    location_id,
    notes,
    updated,
    created,
  };

  records.push(new_record.clone());

  write_all_records(&records);

  println!("Inserted one record:");

  new_record.print_line();
}

fn handle_edit(matches: &ArgMatches<'_>) {
  let contents = matches.values_of("CONTENT").unwrap().collect::<Vec<&str>>();
  let id_str = contents[0];
  let rest_contents: Vec<&str> = contents.iter().skip(1).cloned().collect();
  let full_contents: Vec<String> = get_full_contents(&rest_contents);

  // if first content is '' it would not have any chars
  let first_content_chars = rest_contents[0].chars().take(1).collect::<Vec<char>>();
  if !first_content_chars.is_empty() && first_content_chars[0] == '$' {
    panic!("Unexpected $ char as first item in edit");
  }

  let mut records = get_data_records();
  let context = get_context(&records);
  let what_id = id_str
    .parse::<usize>()
    .expect("You need to pass an id as first argument");

  if context.id_to_str_map.get(&what_id).is_none()
    || context.id_to_record_idx_map.get(&what_id).is_none()
  {
    if context.hierarchy.get(&what_id).is_none() || full_contents.len() != 1 {
      panic!("Unexisting id {}", what_id);
    }

    // the edit is a rename of a location in 1..n records
    let old_location_id = what_id;

    let mut new_location = full_contents[0].clone();
    let new_location_id: usize;

    if let Ok(val) = new_location.parse::<usize>() {
      new_location_id = val;
      new_location = context.id_to_str_map[&new_location_id].clone();
    } else {
      let maybe_location_id = context.str_to_id_map.get(&new_location);
      new_location_id = if maybe_location_id.is_some() {
        *maybe_location_id.unwrap()
      } else {
        context.max_id + 1
      };
    }

    for record in records.iter_mut() {
      if record.location_id == old_location_id {
        record.location_id = new_location_id;
        record.location = new_location.clone();
      }
    }
  } else {
    let record_idx = context.id_to_record_idx_map[&what_id];
    let mut new_what = full_contents[0].clone();
    let mut new_what_id = what_id;

    if !get_is_empty_text(&new_what) && new_what.as_str() != records[record_idx].what {
      if let Ok(val) = new_what.parse::<usize>() {
        new_what_id = val;
        new_what = context.id_to_str_map[&new_what_id].clone();
      }

      if context.str_to_id_map.get(&new_what).is_some() {
        panic!("Existing new what: {}", new_what);
      }

      for (idx, _) in records.clone().iter().enumerate() {
        if records[idx].what_id == what_id {
          records[idx].what = new_what.clone();
          records[idx].what_id = new_what_id;
        } else if records[idx].location_id == what_id {
          records[idx].location = new_what.clone();
        }
      }
    }

    if full_contents.len() > 1 {
      let mut new_location = full_contents[1].clone();

      if !get_is_empty_text(&new_location) && new_location.as_str() != records[record_idx].location
      {
        let new_location_id;

        if let Ok(val) = new_location.parse::<usize>() {
          new_location_id = val;
          new_location = context.id_to_str_map[&new_location_id].clone();
        } else {
          let maybe_location_id = context.str_to_id_map.get(&new_location);
          new_location_id = if maybe_location_id.is_some() {
            *maybe_location_id.unwrap()
          } else {
            context.max_id + 1
          };
        }

        records[record_idx].location = new_location;
        records[record_idx].location_id = new_location_id;
      }
    }

    if full_contents.len() > 2 {
      let mut new_notes = full_contents[2].clone();

      if !get_is_empty_text(&new_notes) {
        if new_notes.as_str() == "-" {
          new_notes = get_empty_notes_text();
        }

        records[record_idx].notes = new_notes;
      }
    }

    records[record_idx].updated = get_now_date();
  }

  println!("Record(s) updated correctly");

  write_all_records(&records)
}

fn handle_remove(matches: &ArgMatches<'_>) {
  let contents = matches.values_of("CONTENT").unwrap().collect::<Vec<&str>>();
  let id_str = contents[0];
  let what_id = id_str
    .parse::<usize>()
    .expect("You need to pass an id as first argument");

  let mut records = get_data_records();
  let context = get_context(&records);
  let record_idx = *context
    .id_to_record_idx_map
    .get(&what_id)
    .expect("Unexisting id");

  records.remove(record_idx);

  println!("Record removed correctly");

  write_all_records(&records)
}

fn handle_stats() {
  let records = get_data_records();
  let context = get_context(&records);
  let mut root_nodes_num = 0;

  for id in context.hierarchy.keys() {
    if context.hierarchy[id].parent.is_none() {
      root_nodes_num += 1;
    }
  }

  println!("Stats:");
  println!("- Count: {}", records.len());
  println!("- Root nodes: {}", root_nodes_num);
}

fn handle_optimize_data() {
  let mut records = get_data_records();

  optimize_records_ids(&mut records);

  println!("Data was optimized successfully.");

  write_all_records(&records)
}

fn handle_list(matches: &ArgMatches<'_>) {
  let records = get_data_records();
  let node_type = matches.value_of("node-type").unwrap_or("all");

  match node_type {
    "all" | "root" | "leaf" => {}
    _ => {
      panic!("Unknown passed node type");
    }
  }

  if node_type == "all" {
    for record in records {
      record.print_line();
    }

    return;
  }

  let context = get_context(&records);

  if node_type == "root" {
    let mut printed_ids: HashSet<usize> = HashSet::new();

    for record in records {
      if context.hierarchy[&record.location_id].parent.is_none()
        && !printed_ids.contains(&record.location_id)
      {
        record.print_location_with_count(&context);
        printed_ids.insert(record.location_id);
      }
    }
  } else if node_type == "leaf" {
    for record in records {
      if context.hierarchy[&record.what_id].children.is_empty() {
        record.print_line();
      }
    }
  }
}

fn handle_tree() {
  fn print_recursive(
    record_id: usize,
    context: &Context,
    records: &[Record],
    depth: usize,
  ) -> usize {
    let str = context.id_to_str_map[&record_id].clone();
    let mut last_depth = depth;
    let tree_node = context.hierarchy.get(&record_id);
    let has_children = match tree_node {
      None => false,
      Some(v) => !v.children.is_empty(),
    };
    let prefix = if has_children { "+" } else { "-" };

    println!(
      "{}{} {} [{}]",
      " ".repeat(depth * 6),
      prefix,
      str,
      record_id
    );

    if tree_node.is_some() {
      for (idx, child_id) in tree_node.unwrap().children.iter().enumerate() {
        if idx == 0 {
          last_depth = depth + 1;
        }

        if last_depth != depth + 1 {
          println!();
        }

        last_depth = print_recursive(*child_id, &context, &records, depth + 1);
      }
    }

    last_depth
  }

  let records = get_data_records();
  let context = get_context(&records);

  println!("<top>");

  let mut depth = 1;
  for id in context.hierarchy.keys() {
    if context.hierarchy[id].parent.is_none() {
      if depth != 1 {
        println!();
      }

      depth = print_recursive(*id, &context, &records, 1);
    }
  }
}

fn parse_args() {
  let mut app = App::new("o")
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
      SubCommand::with_name("se")
        .about("Search")
        .arg(
          Arg::with_name("skip-what")
            .long("skip-what")
            .short("w")
            .help("Skips what from search"),
        )
        .arg(
          Arg::with_name("skip-location")
            .long("skip-location")
            .short("l")
            .help("Skips location from search"),
        )
        .arg(Arg::with_name("CONTENT").multiple(true)),
    )
    .subcommand(
      SubCommand::with_name("in")
        .about("Insert")
        .arg(Arg::with_name("CONTENT").multiple(true)),
    )
    .subcommand(
      SubCommand::with_name("ed")
        .about("Edit")
        .arg(Arg::with_name("CONTENT").multiple(true)),
    )
    .subcommand(
      SubCommand::with_name("rm")
        .about("Remove")
        .arg(Arg::with_name("CONTENT").multiple(true)),
    )
    .subcommand(SubCommand::with_name("st").about("Stats"))
    .subcommand(SubCommand::with_name("optimize-data").about("Optimize data"))
    .subcommand(SubCommand::with_name("tree").about("Display in a tree fashion"))
    .subcommand(
      SubCommand::with_name("ls").about("List").arg(
        Arg::with_name("node-type")
          .long("node-type")
          .short("n")
          .value_name("VALUE")
          .help("Node type ['root' | 'leaf' | 'all']"),
      ),
    );

  let matches = app.clone().get_matches();

  if matches.subcommand_matches("init").is_some() {
    init_project();
  } else if let Some(matches) = matches.subcommand_matches("csv") {
    handle_csv(matches);
  } else if let Some(matches) = matches.subcommand_matches("se") {
    handle_search(matches);
  } else if let Some(matches) = matches.subcommand_matches("in") {
    handle_insert(matches);
  } else if let Some(matches) = matches.subcommand_matches("ed") {
    handle_edit(matches);
  } else if let Some(matches) = matches.subcommand_matches("rm") {
    handle_remove(matches);
  } else if matches.subcommand_matches("st").is_some() {
    handle_stats();
  } else if matches.subcommand_matches("optimize-data").is_some() {
    handle_optimize_data();
  } else if matches.subcommand_matches("tree").is_some() {
    handle_tree();
  } else if let Some(matches) = matches.subcommand_matches("ls") {
    handle_list(matches);
  } else {
    app.print_help().unwrap();
  }
}

fn main() {
  parse_args();
}
