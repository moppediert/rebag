mod bag_indexer;

use bag_indexer::{get_topics, read_bag, get_messages};
use std::env;

fn main() {
    let bag = read_bag(
        env::current_dir()
            .unwrap()
            .as_path()
            .join("example-2.bag")
            .as_path(),
    );
    let mut topics = get_topics(&bag);
    topics.sort();

    // topics.into_iter().for_each(|x| println!("{:?}", x));

    let missions = get_messages(&bag, "/mcu/as_state");
    missions.into_iter().for_each(|x| println!("{:?}", x));
}
