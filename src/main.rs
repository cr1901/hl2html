mod parser;
mod ast;

use parser::parse_hotlist_from_file;

fn main() {
    let foo = parse_hotlist_from_file("").unwrap();
}
