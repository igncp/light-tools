use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub encryption_key: String,
  pub saved_actions: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
  pub created: String,
  pub location: String,
  pub location_id: usize,
  pub notes: String,
  pub updated: String,
  pub what: String,
  pub what_id: usize,
}

impl Record {
  pub fn print_line(&self) {
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

  pub fn print_location_with_count(&self, context: &Context) {
    let count = context.hierarchy[&self.location_id].children.len();

    print!("- {}", &self.location);
    print!(" [{}]", &self.location_id);
    print!(" <{} items>", count);
    println!();
  }
}

#[derive(Debug, Clone)]
pub struct TreeNode {
  pub children: HashSet<usize>,
  pub parent: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Context {
  pub id_to_str_map: HashMap<usize, String>,
  pub str_to_id_map: HashMap<String, usize>,
  pub id_to_record_idx_map: HashMap<usize, usize>,
  pub hierarchy: HashMap<usize, TreeNode>,
  pub max_id: usize,
}

pub fn get_context(records: &[Record]) -> Context {
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
