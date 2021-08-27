mod parser;
mod hotlist;

use parser::{HotlistParser, parse_hotlist_from_file};

fn main() {
    let foo = parse_hotlist_from_file("").unwrap();
}
