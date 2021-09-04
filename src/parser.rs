use crate::ast;
use crate::lexer;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub hotlist); // synthesized by LALRPOP

pub fn parse_hotlist_from_file<'a, T: AsRef<Path>>(filename: T, in_buf: &'a mut String) -> Result<ast::Hotlist<'a>, Box<dyn Error + Send + Sync + 'a>> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);

    buf_reader.read_to_string(&mut *in_buf)?;

    let lexer = lexer::Lexer::new(&*in_buf);
    let parser = hotlist::HotlistParser::new();

    let hotlist = parser.parse(in_buf, lexer)?;

    Ok(hotlist)
}

#[cfg(test)]
mod tests {
    use super::hotlist;
    use crate::ast;
    use crate::lexer;

    use chrono::{Utc, TimeZone};
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
            (
                RefVersion::from("2.0").unwrap(),
                ast::Options {
                    encoding: ast::Encoding::Utf8(RefVersion::from("3.0").unwrap())
                },
            )
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
                timestamp: Utc.timestamp(1322363353, 0)
            }
        );
    }

    #[test]
    fn test_two_notes() {
        let inp = "#NOTE\n\
        \tID=1\n\
        \tUNIQUEID=00000000000000000000000000000000\n\
        \tNAME=Foo.\n\
        \tURL=https://www.example.com/a/random/path\n\
        \tCREATED=0\n\
        \n\
        #NOTE\n\
        \tID=2\n\
        \tUNIQUEID=FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF\n\
        \tNAME=Bar.\n\
        \tURL=http://www.example.org/path/to/file\n\
        \tCREATED=2147483647\n";

        let lexer = lexer::Lexer::new(inp);
        assert_eq!(
            &hotlist::HotlistEntriesParser::new()
                .parse(inp, lexer)
                .unwrap(),
            &[
                ast::EntryKind::Note(
                    ast::Note {
                        id: 1,
                        uuid: Uuid::parse_str("00000000000000000000000000000000").unwrap(),
                        contents: "Foo.",
                        url: Url::parse("https://www.example.com/a/random/path").unwrap(),
                        timestamp: Utc.timestamp(0, 0)
                    }
                ),

                ast::EntryKind::Note(
                    ast::Note {
                        id: 2,
                        uuid: Uuid::parse_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap(),
                        contents: "Bar.",
                        url: Url::parse("http://www.example.org/path/to/file").unwrap(),
                        timestamp: Utc.timestamp(2147483647, 0)
                    }
                ),
            ]
        );
    }
}
