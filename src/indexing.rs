use std::collections::{HashMap, HashSet};
use std::fs;
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

pub fn get_topics(bag: &RosBag) -> Vec<String> {
    let mut topics = vec![];
    // Iterate over records in the index section
    for record in bag.index_records() {
        match record.unwrap() {
            IndexRecord::Connection(conn) => {
                topics.push(conn.topic.to_string());
            }
            _ => {}
        }
    }
    topics
}

pub fn get_messages(bag: &RosBag, topic: &str) -> Vec<Vec<u8>> {
    let mut conn_id = 9999;
    let mut messages = vec![];
    let mut sections = "".to_string();
    let mut num_as_states = 0;
    let mut conns = HashSet::new();
    let start = Instant::now();
    for record in bag.chunk_records() {
        match record.unwrap() {
            ChunkRecord::Chunk(chunk) => {
                for message in chunk.messages() {
                    match message.unwrap() {
                        MessageRecord::MessageData(message_data) => {
                            if message_data.conn_id == 0 {
                                messages.push(message_data.data.into());
                            }
                        }
                        MessageRecord::Connection(conn) => {
                            if conn.topic == topic {
                                conn_id = conn.id;
                            }
                        }
                    }
                }
            }
            ChunkRecord::IndexData(index_data) => {
                conns.insert(index_data.conn_id);
                if index_data.conn_id == conn_id {
                    num_as_states += index_data.count;
                }
            }
        }
    }
    let mut conns = conns.into_iter().collect::<Vec<u32>>();
    conns.sort();
    // println!("{:?}", conns);

    let mut index_section = "".to_string();
    for record in bag.index_records() {
        match record.unwrap() {
            // connection records always come first so conn_id can be read before the first chunk info appears
            IndexRecord::Connection(conn) => {
                println!("{}", conn.message_definition);
                println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
                if conn.topic == topic {
                    conn_id = conn.id;
                }
            }
            IndexRecord::ChunkInfo(chunk_info) => {
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
        }
    }
    let end = Instant::now();
    let duration = end - start;
    // println!("{}", duration.as_secs_f32());
    // println!("{}", sections);
    // println!("-----------------------------------------------------------------------------------------------");
    // println!("{}", index_section);
    // println!("{:?}", num_as_states);
    // println!("{:?}", messages.len());
    // println!("chunk count: {}", bag.get_chunk_count());
    messages
}
