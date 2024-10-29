mod indexing;
pub mod message_parsing;

use indexing::{get_messages, read_bag};
use std::env;

fn main() {
    let bag = read_bag(
        env::current_dir()
            .unwrap()
            .as_path()
            .join("bags/autocross_2023-08-18-17-00-45.bag")
            .as_path(),
    );

    let missions = get_messages(&bag, "/viz/slam/finish_line");
    let string = missions
        .into_iter()
        .map(|v| v.into_iter().map(|x| char::from(x)).collect::<String>())
        .collect::<Vec<String>>();
    println!("{:?}", string.len());
}
