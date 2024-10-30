use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::iter::Map;
use std::path::Path;
use std::time::Instant;

use rosbag::{ChunkRecord, IndexRecord, MessageRecord, RosBag};

pub fn read_bags(path: &Path) -> HashMap<String, RosBag> {
    let paths = fs::read_dir(path).unwrap();
    let mut bag_paths = vec![];

    for path in paths {
        let p = path.unwrap().path();
        if let Some(bag) = p.extension() {
            if bag.to_str().unwrap() == "bag" {
                bag_paths.push(p);
            }
        }
    }

    let mut bags = HashMap::<String, RosBag>::new();

    for bag_path in bag_paths {
        let bag_path_str = bag_path.file_name().unwrap().to_str().unwrap();
        let bag = RosBag::new(bag_path_str).unwrap();

        bags.insert(bag_path_str.to_string(), bag);
    }
    bags
}

pub fn read_bag(path: &Path) -> RosBag {
    let bag_path_str = path.to_str().unwrap();
    RosBag::new(bag_path_str).unwrap()
}

pub fn get_messages(bag: &RosBag, topic: &str) -> Vec<Vec<u8>> {
    let mut conn_id = 9999;
    let mut messages = vec![];
    let mut sections = "".to_string();
    let mut num_as_states = 0;
    let mut conns = HashSet::new();
    let mut index_section = "".to_string();
    let start = Instant::now();

    // Chunk records contain connection and message records
    for record in bag.chunk_records() {
        match record {
            Ok(ChunkRecord::Chunk(chunk)) => {
                index_section = format!("{}/{}", index_section, chunk.messages().count());
                for message in chunk.messages() {
                    match message {
                        Ok(MessageRecord::Connection(conn)) => {
                            if conn.topic == topic {
                                conn_id = conn.id;
                            }
                        }
                        Ok(MessageRecord::MessageData(message_data)) => {
                            if message_data.conn_id == 0 {
                                messages.push(message_data.data.into());
                            }
                        }
                        Err(_) => todo!(),
                    }
                }
            }
            Ok(ChunkRecord::IndexData(index_data)) => {
                index_section = format!("{}/{}", index_section, "X");
                conns.insert(index_data.conn_id);
                if index_data.conn_id == conn_id {
                    num_as_states += index_data.count;
                }
            }
            Err(_) => todo!(),
        }
    }

    for record in bag.index_records() {
        match record {
            // connection records always come first so conn_id can be read before the first chunk info appears
            Ok(IndexRecord::Connection(conn)) => {
                index_section = format!("{}/{}", index_section, "C");
                if conn.topic == topic {
                    conn_id = conn.id;
                }
            }
            Ok(IndexRecord::ChunkInfo(chunk_info)) => {
                index_section = format!("{}/{}", index_section, "I");
                let num_local_as_states = chunk_info
                    .entries()
                    .filter_map(|e| {
                        if e.conn_id == conn_id {
                            Some(e.count)
                        } else {
                            None
                        }
                    })
                    .sum::<u32>();
                num_as_states += num_local_as_states;
            }
            Err(_) => todo!(),
        }
    }
    let end = Instant::now();
    let duration = end - start;
    // println!("{}", duration.as_secs_f32());
    // println!("{}", sections);
    // println!("-----------------------------------------------------------------------------------------------");
    // println!("{:?}", num_as_states);
    // println!("{:?}", messages.len());
    // println!("chunk count: {}", bag.get_chunk_count());
    messages
}

pub fn get_topics(bag: &RosBag) -> Vec<&str> {
    let mut result = vec![];
    for record in bag.index_records() {
        match record {
            Ok(IndexRecord::Connection(conn)) => {
                result.push(conn.topic);
            }

            Ok(IndexRecord::ChunkInfo(_)) => {}
            Err(_) => todo!(),
        }
    }
    result
}

pub fn get_message_count(bag: &RosBag) -> BTreeMap<&str, u64> {
    let mut conn_id_to_topic = BTreeMap::new();
    let mut count = BTreeMap::new();
    for record in bag.index_records() {
        match record {
            Ok(IndexRecord::Connection(conn)) => {
                conn_id_to_topic.insert(conn.id, conn.topic);
                count.insert(conn.topic, 0);
            }
            Ok(IndexRecord::ChunkInfo(chunk_info)) => {
                for entry in chunk_info.entries() {
                    count
                        .entry(conn_id_to_topic.get(&entry.conn_id).unwrap())
                        .and_modify(|count| *count += 1);
                }
            }
            Err(_) => todo!(),
        }
    }
    count
}
