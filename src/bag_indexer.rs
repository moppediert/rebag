use rosbag::{IndexRecord, RosBag};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

pub fn get_messages(bag: &RosBag, topic: &str) -> Vec<String> {
    let mut conn_id = 0;
    let mut messages = vec![];
    for record in bag.chunk_records() {
        if record.is_err() {
            continue;
        }
        match record.unwrap() {
            rosbag::ChunkRecord::Chunk(chunk) => {
                for message in chunk.messages() {
                    match message.unwrap() {
                        rosbag::MessageRecord::MessageData(message_data) => {
                            if message_data.conn_id == conn_id {
                                messages.push(format!("{:?}", message_data.data));
                            }
                        }
                        rosbag::MessageRecord::Connection(conn) => {
                            if conn.topic == topic {
                                conn_id = conn.id;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    messages
}
