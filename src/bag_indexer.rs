use rosbag::{IndexRecord, RosBag};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn read_bags(path: &Path) -> HashMap<String, Vec<String>> {
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

    let mut bags = HashMap::<String, Vec<String>>::new();

    for bag_path in bag_paths {
        let bag_path_str = bag_path.file_name().unwrap().to_str().unwrap();
        let bag = RosBag::new(bag_path_str).unwrap();

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
        bags.insert(bag_path_str.to_string(), topics);
    }
    bags
}
