use lexgen::lexer;

use std::error;
use std::fmt;

use crate::ast;

// It's unfortunate, but the lexer has to do a bit of it's own parsing to successfully parse
// notes, since the value of NAME can be essentially "anything except a newline". By default,
// the LALRPOP lexer tries to match the longest option, and that would basically mean "everything
// is a NameBody".

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tok<'input> {
    // Literals
    HotlistVersion,  // "Opera Hotlist version"
    Options,         // "Options: "
    Encoding,        // "encoding"
    EncodingVersion, // "version"
    Utf8,            // "utf8"
    Equal,           // "="
    NoteHeader,      // "#NOTE"
    Id,              // "ID"
    UniqueId,        // "UNIQUEID"
    Name,            // "NAME"
    Url,             // "URL"
    Created,         // "CREATED"
    Comma,           // ","
    Expanded,        // "EXPANDED"
    TrashFolder,     // "TRASH FOLDER"
    Yes,             // "YES"
    No,              // "NO"
    FolderHeader,    // "#FOLDER"
    FolderEnd,       // "-"
    Active,          // "ACTIVE"

    // Regex-based
    Version(&'input str),
    Integer(&'input str),
    Uuid(&'input str),
    UrlBody(&'input str),
    NoteBody(&'input str),
}

impl<'input> fmt::Display for Tok<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tok::HotlistVersion => write!(f, r#"Hotlist version ("Opera Hotlist version")"#),
            Tok::Options => write!(f, r#"Options ("Options:")"#),
            Tok::Encoding => write!(f, r#"Encoding ("encoding")"#),
            Tok::EncodingVersion => write!(f, r#"Encoding version ("version")"#),
            Tok::Utf8 => write!(f, r#"UTF8 ("utf8")"#),
            Tok::Equal => write!(f, r#"Equals sign ("=")"#),
            Tok::NoteHeader => write!(f, r##"Note header ("#NOTE")"##),
            Tok::Id => write!(f, r#"Numeric ID field ("ID")"#),
            Tok::UniqueId => write!(f, r#"UUID field ("UNIQUEID")"#),
            Tok::Name => write!(f, r#"Note body field ("NAME")"#),
            Tok::Url => write!(f, r#"URL field ("URL")"#),
            Tok::Created => write!(f, r#"Timestamp field ("CREATED")"#),
            Tok::Comma => write!(f, r#"Comma (",")"#),
            Tok::Expanded => write!(f, r#"Expanded ("EXPANDED")"#),
            Tok::TrashFolder => write!(f, r#"Trash folder ("TRASH FOLDER")"#),
            Tok::Yes => write!(f, r#"Yes/True ("YES")"#),
            Tok::No => write!(f, r#"No/False ("NO")"#),
            Tok::FolderHeader => write!(f, r##"Folder header ("#FOLDER")"##),
            Tok::FolderEnd => write!(f, r#"End of folder delimiter ("-")"#),
            Tok::Active => write!(f, r#"Active ("ACTIVE")"#),

            // Regex-based
            Tok::Version(ver) => write!(f, r#"Version ("{}")"#, ver),
            Tok::Integer(int) => write!(f, r#"Integer ("{}")"#, int),
            Tok::Uuid(uuid) => write!(f, r#"UUID ("{}")"#, uuid),
            Tok::UrlBody(url) => write!(f, r#"URL ("{}")"#, url),
            Tok::NoteBody(note) => {
                if note.len() < 80 {
                    write!(f, r#"Note body ("{}")"#, note)
                } else {
                    write!(f, r#"Note body ("{}"... [cont])"#, note)
                }
            }
        }
    }
}

#[derive(Default)]
pub struct LexerState {
    in_note_name: bool,
}

lexer! {
    pub Lexer(LexerState) -> Tok<'input>;
    type Error<'input> = ast::HotlistError<'input>;

    let version_re = ['0'-'9']+ ('.'['0'-'9'])? ['0'-'9']*;
    let integer_re = ['0'-'9']*;
    let uuid_re = ['0'-'9' 'A'-'F']*;
    // Avoid conflicts with literals like "utf8," by only parsing a subset of valid URLs.
    let url_re = ("http" | "https") ($$ascii_alphanumeric | ":" | "/" | "." | "-" | "~" | "_" | "#" | "$" | "," |
        ";" | "(" | ")" | "'" | "?" | "[" | "]" | "@" | "!" | "&" | "*" | "+" | "=" |
        ("%" ['0'-'9' 'A'-'F'] ['0'-'9' 'A'-'F']))+;

    // Rule for everything except slurping up note body and URLs.
    rule Init {
        // Whitespace should be skipped when possible.
        [' ' '\t' '\n' '\r']+,

        "Opera Hotlist version" = Tok::HotlistVersion,
        "Options:" = Tok::Options,
        "encoding" = Tok::Encoding,
        "version" = Tok::EncodingVersion,
        "utf8" = Tok::Utf8,

        "=" => |mut lexer| {
            if lexer.state().in_note_name {
                lexer.switch_and_return(LexerRule::NoteBody, Tok::Equal)
            } else {
                lexer.return_(Tok::Equal)
            }
        },

        "#NOTE" = Tok::NoteHeader,
        "ID" = Tok::Id,
        "UNIQUEID" = Tok::UniqueId,

        "NAME" => |mut lexer| {
             lexer.state().in_note_name = true;
             lexer.return_(Tok::Name)
        },

        "URL" = Tok::Url,
        "CREATED" = Tok::Created,
        "," = Tok::Comma,
        "EXPANDED" = Tok::Expanded,
        "TRASH FOLDER"  = Tok::TrashFolder,
        "YES"  = Tok::Yes,
        "NO" = Tok::No,
        "#FOLDER" = Tok::FolderHeader,
        "-" = Tok::FolderEnd,
        "ACTIVE" = Tok::Active,

        // Regexes
        $integer_re => |lexer| {
            let match_ = lexer.match_();
            lexer.return_(Tok::Integer(match_))
        },

        $version_re => |lexer| {
            let match_ = lexer.match_();
            lexer.return_(Tok::Version(match_))
        },

        $uuid_re => |lexer| {
            let match_ = lexer.match_();
            lexer.return_(Tok::Uuid(match_))
        },

        $url_re => |lexer| {
            let match_ = lexer.match_();
            lexer.return_(Tok::UrlBody(match_))
        },
    }

    // Chomp characters until a newline is found!
    rule NoteBody {
        _ => |mut lexer| {
            if let Some('\n') = lexer.peek() {
                let match_ = lexer.match_();
                lexer.state().in_note_name = false;

                // We don't want to match CR either, but only if it precedes a \n.
                let match_end = if match_.rfind('\r') == Some(match_.len() - 1) {
                    match_.len() - 2
                } else {
                    match_.len() - 1
                };

                // For some reason the equal sign remains, so we remove it here.
                // TODO: Handle empty slice.
                lexer.switch_and_return(LexerRule::Init, Tok::NoteBody(&match_[1..=match_end]))
            } else {
                lexer.continue_()
            }
        },
    }
}

impl<'input> fmt::Display for LexerError<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::LexerError { char_idx } => {
                write!(f, "unknown token starting at offset {}", char_idx)
            }
            LexerError::UserError(e) => e.fmt(f),
        }
    }
}

impl error::Error for LexerError<'static> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            LexerError::UserError(e) => Some(e),
            _ => None,
        }
    }
}
