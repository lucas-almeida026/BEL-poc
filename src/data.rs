use std::collections::HashMap;

use serde_binary::binary_stream::Endian;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Entry {
    pub item_id: u32,
    pub group_id: u32,
}
impl Entry {
    pub fn new(item_id: u32, group_id: u32) -> Self {
        Entry { item_id, group_id }
    }
}

pub fn group_entries(entries: &Vec<Entry>) -> HashMap<u32, Vec<&Entry>> {
    let mut map = HashMap::new();
    for entry in entries {
        map.entry(entry.group_id)
            .and_modify(|v: &mut Vec<&Entry>| v.push(entry))
            .or_insert(vec![entry]);
    }
    return map;
}

pub fn build_relationship_map(entries: &Vec<&Entry>) -> HashMap<u32, HashMap<u32, u32>> {
    let mut map = HashMap::new();
    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            let entry_a = entries[i];
            let entry_b = entries[j];

            if entry_a.item_id == entry_b.item_id {
                continue;
            }

            // Increment weight for entry_a -> entry_b
            map.entry(entry_a.item_id)
                .or_insert_with(HashMap::new)
                .entry(entry_b.item_id)
                .and_modify(|weight| *weight += 1)
                .or_insert(1);

            // Increment weight for entry_b -> entry_a
            map.entry(entry_b.item_id)
                .or_insert_with(HashMap::new)
                .entry(entry_a.item_id)
                .and_modify(|weight| *weight += 1)
                .or_insert(1);
        }
    }
    map
}

pub fn edge_list_map_from_relationship_map(
    map: HashMap<u32, HashMap<u32, u32>>,
) -> HashMap<u32, Vec<(u32, u32)>> {
    let mut m = HashMap::new();
    for (key, submap) in map {
        for (other, weight) in submap {
            m.entry(key)
                .and_modify(|v: &mut Vec<(u32, u32)>| v.push((other, weight)))
                .or_insert(vec![(other, weight)]);
        }
    }
    m
}

pub fn merge_edge_lists(main_list: &mut Vec<(u32, u32)>, diff_list: &Vec<(u32, u32)>) {
    for (id, w) in diff_list {
        match main_list.iter_mut().find(|(id_t, _)| id_t == id) {
            Some(t) => {
                t.1 += w;
            }
            None => {
                main_list.push((*id, *w));
            }
        }
    }
}

pub fn serialize_edge_list(edge_list: Vec<(u32, u32)>) -> Result<Vec<u8>, Error> {
    match serde_binary::to_vec(&edge_list, Endian::Little) {
		Ok(r) => Ok(r),
		Err(_) => Err(Error::new(ErrorKind::InvalidData, "oh no"))
	}
}

pub fn deserialize_edge_list(data: Vec<u8>) -> Result<Vec<(u32, u32)>, Error> {
    match serde_binary::from_slice(&data, Endian::Little) {
        Ok(r) => Ok(r),
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "oh no")),
    }
}
