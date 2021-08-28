use crate::hotlist::*;

use std::path::Path;
use std::fs;

#[macro_use] use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub hotlist); // synthesized by LALRPOP

use eyre::Result;

pub fn parse_hotlist_from_file<'a, T: AsRef<Path>>(filename : T) -> Result<HotList<'a>> {
    let hotlist = fs::read_to_string(filename)?;
    // let file = HotlistParser::parse(Rule::HOTLIST, &parser)?.next().unwrap();

    //for entry in file.
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::hotlist;
    use version_compare::version::Version as RefVersion;

    #[test]
    fn test_version() {
        assert_eq!(hotlist::HotlistHeaderParser::new().parse("Opera Hotlist version 2.0").unwrap(), RefVersion::from("2.0").unwrap());
    }
}
