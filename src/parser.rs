use crate::ast::*;

use std::fs;
use std::path::Path;

use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(pub hotlist); // synthesized by LALRPOP

use eyre::Result;

pub fn parse_hotlist_from_file<'a, T: AsRef<Path>>(filename: T) -> Result<HotList<'a>> {
    let hotlist = fs::read_to_string(filename)?;
    // let file = HotlistParser::parse(Rule::HOTLIST, &parser)?.next().unwrap();

    //for entry in file.
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::hotlist;
    use crate::ast;
    use crate::lexer;

    use chrono::{DateTime, Utc, NaiveDateTime};
    use lalrpop_util::ParseError;
    use url::Url;
    use uuid::Uuid;
    use version_compare::version::Version as RefVersion;

    #[test]
    fn test_version() {
        let inp = "Opera Hotlist version 2.0";
        let lexer = lexer::Lexer::new(inp);
        assert_eq!(
            hotlist::HotlistVersionParser::new()
                .parse(inp, lexer)
                .unwrap(),
            RefVersion::from("2.0").unwrap()
        );
    }

    #[test]
    fn test_encoding() {
        let inp = "encoding = utf8, version=3";
        let lexer = lexer::Lexer::new(inp);
        assert_eq!(
            hotlist::SingleOpParser::new().parse(inp, lexer).unwrap(),
            ast::SingleOp::Encoding(ast::Encoding::Utf8(RefVersion::from("3.0").unwrap()))
        );
    }

    #[test]
    fn test_options() {
        let inp = "Options: encoding = utf8, version=3";
        let lexer = lexer::Lexer::new(inp);
        assert_eq!(
            hotlist::HotlistOptionsParser::new()
                .parse(inp, lexer)
                .unwrap(),
            ast::Options {
                encoding: ast::Encoding::Utf8(RefVersion::from("3.0").unwrap())
            }
        );
    }

    #[test]
    fn test_options_missing() {
        let inp = "Options:";
        let lexer = lexer::Lexer::new(inp);
        assert_eq!(
            hotlist::HotlistOptionsParser::new()
                .parse(inp, lexer)
                .unwrap_err(),
            ast::HotlistError::RequiredFieldMissing("encoding").into()
        );
    }

    #[test]
    fn test_header() {
        let inp = "Opera Hotlist version 2.0\n\
                   Options: encoding = utf8, version=3\n";
        let lexer = lexer::Lexer::new(inp);
        assert_eq!(
            hotlist::HotlistHeaderParser::new()
                .parse(inp, lexer)
                .unwrap(),
            ast::HotList {
                version: RefVersion::from("2.0").unwrap(),
                options: ast::Options {
                    encoding: ast::Encoding::Utf8(RefVersion::from("3.0").unwrap())
                },
                entries: Vec::<ast::EntryKind>::new()
            }
        );
    }

    #[test]
    fn test_note_with_linebreak() {
        let inp = "#NOTE\n\
        \tID=18\n\
        \tUNIQUEID=75356378DB08C2429F4BE860ED92596F\n\
        \tNAME=This is a fake note with \x02\x02an encoded linebreak.\n\
        \tURL=http://www.example.com\n\
        \tCREATED=1322363353\n";

        let lexer = lexer::Lexer::new(inp);
        assert_eq!(
            hotlist::NoteEntryParser::new()
                .parse(inp, lexer)
                .unwrap(),
            ast::Note {
                id: 18,
                uuid: Uuid::parse_str("75356378DB08C2429F4BE860ED92596F").unwrap(),
                contents: "This is a fake note with \x02\x02an encoded linebreak.",
                url: Url::parse("http://www.example.com").unwrap(),
                timestamp: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1322363353, 0), Utc)
            }
        );
    }
}
