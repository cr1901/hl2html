use lexgen::lexer;

// It's unfortunate, but the lexer has to do a bit of it's own parsing to successfully parse
// notes, since the value of NAME can be essentially "anything except a newline". By default,
// the lexer tries to match the longest option, and that would basically mean "everything is a
// NameBody".

#[derive(Copy, Clone, Debug)]
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

    // Regex-based
    Version(&'input str),
    Integer(&'input str),
    Uuid(&'input str),
    UrlBody(&'input str),
    NoteBody(&'input str),
}

lexer! {
    pub Lexer -> Tok<'input>;

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

        "Opera Hotlist version" => |lexer| {
            lexer.return_(Tok::HotlistVersion)
        },

        "Options:" => |lexer| {
            lexer.return_(Tok::Options)
        },

        "encoding" => |lexer| {
            lexer.return_(Tok::Encoding)
        },

        "version" => |lexer| {
            lexer.return_(Tok::EncodingVersion)
        },

        "utf8" => |lexer| {
            lexer.return_(Tok::Utf8)
        },

        "=" => |lexer| {
            lexer.return_(Tok::Equal)
        },

        "#NOTE" => |lexer| {
             lexer.return_(Tok::NoteHeader)
        },

        "ID" => |lexer| {
             lexer.return_(Tok::Id)
        },

        "UNIQUEID" => |lexer| {
             lexer.return_(Tok::UniqueId)
        },

        "NAME" => |lexer| {
             lexer.return_(Tok::Name)
        },

        "URL" => |lexer| {
             lexer.return_(Tok::Url)
        },

        "CREATED" => |lexer| {
             lexer.return_(Tok::Created)
        },

        "," => |lexer| {
             lexer.return_(Tok::Comma)
        },

        // Regexes
        $version_re => |lexer| {
            let match_ = lexer.match_();
            lexer.return_(Tok::Version(match_))
        },

        $integer_re => |lexer| {
            let match_ = lexer.match_();
            lexer.return_(Tok::Integer(match_))
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
}
