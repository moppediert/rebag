use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

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
    let mut conn_id = u32::max_value();
    let mut messages = vec![];

    // Get connection id from topic name
    for record in bag.index_records() {
        match record {
            // connection records always come first so conn_id can be read before the first chunk info appears
            Ok(IndexRecord::Connection(conn)) => {
                if conn.topic == topic {
                    conn_id = conn.id;
                    break;
                }
            }
            Ok(IndexRecord::ChunkInfo(_)) => {}
            Err(_) => todo!(),
        }
    }

    if conn_id == u32::MAX {
        todo!("Topic not found")
    }

    // Chunk records contain connection and message records
    for record in bag.chunk_records() {
        match record {
            Ok(ChunkRecord::Chunk(chunk)) => {
                for message in chunk.messages() {
                    match message {
                        Ok(MessageRecord::Connection(_)) => {}
                        Ok(MessageRecord::MessageData(message_data)) => {
                            if message_data.conn_id == conn_id {
                                messages.push(message_data.data.to_vec());
                            }
                        }
                        Err(_) => todo!(),
                    }
                }
            }
            Ok(ChunkRecord::IndexData(_)) => {}
            Err(_) => todo!(),
        }
    }

    messages
}

type Topic<'a> = &'a str;
type MessageDefinition<'a> = &'a str;
pub fn get_topics(bag: &RosBag) -> BTreeMap<Topic, MessageDefinition> {
    let mut result = BTreeMap::new();
    for record in bag.index_records() {
        match record {
            Ok(IndexRecord::Connection(conn)) => {
                result.insert(conn.topic, conn.message_definition);
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
                chunk_info.entries().for_each(|entry| {
                    count
                        .entry(conn_id_to_topic.get(&entry.conn_id).unwrap())
                        .and_modify(|count| *count += entry.count as u64);
                });
            }
            Err(_) => todo!(),
        }
    }
    count
}
