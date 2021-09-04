use crate::ast;
use crate::lexer;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub hotlist); // synthesized by LALRPOP

pub struct LineInfo {
    pub line: usize,
    pub offset: usize,
}

pub fn parse_hotlist_from_file<'a, T: AsRef<Path>>(
    filename: T,
    in_buf: &'a mut String,
) -> Result<ast::Hotlist<'a>, Box<dyn Error + Send + Sync + 'a>> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);

    buf_reader.read_to_string(&mut *in_buf)?;

    let lexer = lexer::Lexer::new(&*in_buf);
    let parser = hotlist::HotlistParser::new();

    let hotlist = parser.parse(in_buf, lexer)?;

    Ok(hotlist)
}

pub fn get_line_and_offset<T: Read>(
    filebuf: &mut T,
    file_offset: usize,
) -> Result<LineInfo, Box<dyn Error + Send + Sync + 'static>> {
    let mut num_lines = 1;
    let mut offset_cur_line = 1;

    for (i, b) in filebuf.bytes().enumerate() {
        if (b? as char) == '\n' {
            num_lines = num_lines + 1;
            offset_cur_line = i + 1;
        }

        if i >= file_offset {
            break;
        }
    }

    Ok(LineInfo {
        line: num_lines,
        offset: file_offset - offset_cur_line,
    })
}

#[cfg(test)]
mod tests {
    use super::hotlist;
    use crate::ast;
    use crate::lexer;

    use chrono::{TimeZone, Utc};
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
            ast::HotlistError::RequiredFieldMissing("encoding", ast::SpanInfo { error: None, entry: (0, 8)}).into()
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
            hotlist::NoteEntryParser::new().parse(inp, lexer).unwrap(),
            ast::Note {
                id: 18,
                uuid: Uuid::parse_str("75356378DB08C2429F4BE860ED92596F").unwrap(),
                contents: Some("This is a fake note with \x02\x02an encoded linebreak."),
                url: Some(Url::parse("http://www.example.com").unwrap()),
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
                ast::EntryKind::Note(ast::Note {
                    id: 1,
                    uuid: Uuid::parse_str("00000000000000000000000000000000").unwrap(),
                    contents: Some("Foo."),
                    url: Some(Url::parse("https://www.example.com/a/random/path").unwrap()),
                    timestamp: Utc.timestamp(0, 0)
                }),
                ast::EntryKind::Note(ast::Note {
                    id: 2,
                    uuid: Uuid::parse_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap(),
                    contents: Some("Bar."),
                    url: Some(Url::parse("http://www.example.org/path/to/file").unwrap()),
                    timestamp: Utc.timestamp(2147483647, 0)
                }),
            ]
        );
    }
}
