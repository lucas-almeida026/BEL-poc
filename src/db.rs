use std::fs;
use std::io::{Read, Result, Write};
use std::path;
use std::cmp::min;
use crate::data::{self, Entry};

const RELATIVE_FOLDER_PATH: &str = "./repo";

pub fn init_repository() -> Result<()> {
    let dir_path = path::Path::new(RELATIVE_FOLDER_PATH);

    if dir_path.exists() {
        eprintln!("Repo already initialized");
        return Ok(());
    }

    fs::create_dir_all(RELATIVE_FOLDER_PATH)?;
    Ok(())
}

fn read_bin_file(filename: &str) -> Result<Vec<u8>> {
    let mut file = fs::File::open(format!("{RELATIVE_FOLDER_PATH}/{filename}"))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_bin_file(filename: &str, data: &Vec<u8>) -> Result<()> {
    let path_str = format!("{RELATIVE_FOLDER_PATH}/{filename}");
    let mut file = fs::File::create(path_str)?;
    file.write_all(data.as_slice())?;
    Ok(())
}


/* SQL like example query:
// inserting data
put {item: 1, group: 1}, {item: 3, group: 1}, {item: 1, group: 1}

// retreving data
get top 5 from 1
*/


pub fn put(entries: Vec<Entry>) -> Result<()> {
	let groups = data::group_entries(&entries);
	for (_, entries) in groups {
		let rmap = data::build_relationship_map(&entries);
		let emap = data::edge_list_map_from_relationship_map(rmap);
		for (id, diff_list) in emap {
			let filename = format!("{id}.bel");

			if diff_list.is_empty() {
				return Ok(());
			}

			let mut current = Vec::new();
			if let Ok(data) = read_bin_file(&filename) {
				current = data::deserialize_edge_list(data)?;
			}

			data::merge_edge_lists(&mut current, &diff_list);

			let serialized = data::serialize_edge_list(current)?;
			write_bin_file(&filename, &serialized)?;
		}
	}
	Ok(())
}

pub fn get(id: u32, top_k: u32) -> Result<Vec<(u32, u32)>> {
	let filename = format!("{id}.bel");

	let mut edge_list = Vec::new();
	if let Ok(data) = read_bin_file(&filename) {
		edge_list = data::deserialize_edge_list(data)?;
	}

	edge_list.sort_by(|a, b| b.1.cmp(&a.1));
	println!("sorted = {:?}", edge_list);

	let upper_bound = min(edge_list.len(), top_k as usize);

	Ok(edge_list[0..upper_bound].to_vec())
}