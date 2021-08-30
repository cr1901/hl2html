use std::str::CharIndices;

// It's unfortunate, but the lexer has to do a bit of it's own parsing to successfully parse
// notes, since the value of NAME can be essentially "anything except a newline". By default,
// the lexer tries to match the longest option, and that would basically mean "everything is a
// NameBody".

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Copy, Clone, Debug)]
pub enum Tok<'input> {
    // Literals
    HotlistVersion,   // "Opera Hotlist version"
    Options,          // "Options: "
    Encoding,         // "encoding"
    EncodingVersion,  // "version"
    Utf8,             // "utf8"
    Equal,            // "="
    NoteHeader,       // "#NOTE"
    Id,               // "ID"
    UniqueId,         // "UNIQUEID"
    Name,             // "NAME"
    Url,              // "URL"
    Created,          // "CREATED"
    Comma,            // ","

    // Regex-based
    Version(&'input str),
    Timestamp,
    UrlBody,
    NoteBody
}

#[derive(Debug)]
pub enum LexicalError {
    // Not possible
}

pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { chars: input.char_indices() }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
