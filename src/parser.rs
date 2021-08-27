use pest_derive::Parser;
use pest::Parser;
use crate::hotlist::*;

use std::path::Path;
use std::fs;
use failure::Error;

#[derive(Parser)]
#[grammar = "hotlist.pest"]
pub struct HotlistParser;

pub fn parse_hotlist_from_file<'a, T: AsRef<Path>>(filename : T) -> Result<HotList<'a>, Error> {
    let parser = fs::read_to_string(filename)?;
    let file = HotlistParser::parse(Rule::HOTLIST, &parser)?.next().unwrap();

    //for entry in file.
    unimplemented!();
}
