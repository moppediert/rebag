use rosbag::{ChunkRecord, IndexRecord, MessageRecord, RosBag};
use std::env;

fn main() {
    let path = env::current_dir().unwrap().join("example.bag");
    let bag = RosBag::new(path).unwrap();
    // Iterate over records in the chunk section
    for record in bag.chunk_records() {
        match record.unwrap() {
            ChunkRecord::Chunk(chunk) => {
                // iterate over messages in the chunk
                for msg in chunk.messages() {
                    match msg.unwrap() {
                        MessageRecord::MessageData(msg_data) => {
                            // println!("{:?}", msg_data.data);
                        }
                        MessageRecord::Connection(conn) => {
                            println!("{:?}", conn.topic);
                        }
                    }
                }
            }
            ChunkRecord::IndexData(index_data) => {
                for entry in index_data.entries() {
                    // println!("{:?}", entry.time);
                }
            }
        }
    }
    // Iterate over records in the index section
    for record in bag.index_records() {
        match record.unwrap() {
            IndexRecord::IndexData(index_data) => {
                for entry in index_data.entries() {
                    // println!("{:?}", entry.time);
                }
            }
            IndexRecord::Connection(conn) => {
                // println!("{:?}", conn.topic);
            }
            IndexRecord::ChunkInfo(chunk_info) => {
                // println!("{:?}", chunk_info.chunk_pos);
            }
        }
    }
}
