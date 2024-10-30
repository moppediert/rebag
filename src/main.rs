mod indexing;
pub mod message_parsing;

use indexing::{get_message_count, read_bag};
use std::env;
use tabled::{
    settings::{
        themes::{Colorization, ColumnNames},
        Color, Style,
    },
    Table,
};

fn main() {
    let bag = read_bag(
        env::current_dir()
            .unwrap()
            .as_path()
            .join("bags/autocross_2023-08-18-17-00-45.bag")
            .as_path(),
    );

    let message_count = get_message_count(&bag);
    let color_col1 = Color::BG_GREEN | Color::FG_BLACK;
    let color_col2 = Color::BG_MAGENTA | Color::FG_BLACK;
    println!(
        "{}",
        Table::new(message_count)
            .with(ColumnNames::new(["Topic", "Message count"]))
            .with(Style::psql())
            .with(Colorization::columns([color_col1, color_col2]))
    );
}
