use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hotlist.pest"]
pub struct HotlistParser;
