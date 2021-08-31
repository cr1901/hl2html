use lexgen::lexer;

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
    Folder,          // "FOLDER"
    FolderEnd,       // "-"

    // Regex-based
    Version(&'input str),
    Integer(&'input str),
    Uuid(&'input str),
    UrlBody(&'input str),
    NoteBody(&'input str),
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
        [' ' '\t' '\n']+,

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
        "FOLDER" = Tok::Folder,
        "-" = Tok::FolderEnd,

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

                // For some reason the equal sign remains. TODO: Handle empty slice.
                lexer.switch_and_return(LexerRule::Init, Tok::NoteBody(&match_[1..]))
            } else {
                lexer.continue_()
            }
        },
    }
}
