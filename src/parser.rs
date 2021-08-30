use crate::ast::*;

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
    use crate::ast;
    use version_compare::version::Version as RefVersion;

    #[test]
    fn test_version() {
        assert_eq!(hotlist::HotlistVersionParser::new().parse("Opera Hotlist version 2.0").unwrap(), RefVersion::from("2.0").unwrap());
    }

    #[test]
    fn test_encoding() {
        assert_eq!(hotlist::SingleOpParser::new().parse("encoding = utf8, version=3").unwrap(),
            ast::SingleOp::Encoding(ast::Encoding::Utf8(RefVersion::from("3.0").unwrap())));
    }

    #[test]
    fn test_options() {
        assert_eq!(
            hotlist::HotlistOptionsParser::new().parse("Options: encoding = utf8, version=3").unwrap(),
            ast::Options {
                encoding: ast::Encoding::Utf8(RefVersion::from("3.0").unwrap())
            });
    }

    #[test]
    fn test_header() {
        assert_eq!(
            hotlist::HotlistOptionsParser::new().parse("Options: encoding = utf8, version=3").unwrap(),
            ast::Options {
                encoding: ast::Encoding::Utf8(RefVersion::from("3.0").unwrap())
            });
    }
}

//     #[test]
//     fn test_note_with_() {
//         let note = "#NOTE\n\
//         \tID=18\n\
//         \tUNIQUEID=75356378DB08C2429F4BE860ED92596F\n\
//         \tNAME=This is a fake note with \x02\x02an encoded linebreak.\n\
//         \tURL=http://www.example.com\n\
//         \tCREATED=1322363353\n";
//
//         println!("{}", note);
//     }
// }
