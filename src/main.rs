mod ast;
mod lexer;
mod parser;

use parser::parse_hotlist_from_file;

fn main() {
    let mut in_buf = String::new();

    let hotlist = parse_hotlist_from_file("", &mut in_buf).unwrap();

    unimplemented!()
}
