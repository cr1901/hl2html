use chrono::{DateTime, Utc};
use lalrpop_util::ParseError;
use url::Url;
use uuid::Uuid;
use version_compare::version::Version;

use crate::lexer::{LexerError, Tok};

#[derive(Debug, PartialEq)]
pub struct Hotlist<'a> {
    pub version: Version<'a>,
    pub options: Options<'a>,
    pub entries: Vec<EntryKind<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Options<'a> {
    pub encoding: Encoding<'a>,
}

// For now, assign default values to omitted options. Last value in repeated list takes priority.
#[derive(Debug, PartialEq)]
pub(crate) enum SingleOp<'a> {
    Encoding(Encoding<'a>),
}

#[derive(Debug, PartialEq)]
pub enum Encoding<'a> {
    Utf8(Version<'a>),
}

#[derive(Debug, PartialEq)]
pub enum EntryKind<'a> {
    Folder(Folder<'a>),
    Note(Note<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Folder<'a> {
    pub id: u32,
    pub uuid: Uuid,
    pub name: &'a str,
    pub timestamp: DateTime<Utc>,
    pub trash: bool,
    pub expanded: bool,
    pub entries: Vec<EntryKind<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Note<'a> {
    pub id: u32,
    pub uuid: Uuid,
    pub contents: &'a str,
    pub url: Url,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum NoteField<'a> {
    Id(u32),
    Uuid(Uuid),
    Contents(&'a str),
    Url(Url),
    Timestamp(DateTime<Utc>),
}

// TODO: impl Display.
// We squirrel this away in LexerError's UserError variant, because LexerError is already
// associated with the ParseError::User variant.
#[derive(Debug, PartialEq, Eq)]
pub enum HotlistError<'a> {
    RequiredFieldMissing(&'a str),
    U32OutOfRange(&'a str),
    InvalidUuid(&'a str),
    InvalidUrl(&'a str),
}

impl<'a> From<HotlistError<'a>> for ParseError<usize, Tok<'_>, LexerError<'a>> {
    fn from(error: HotlistError<'a>) -> Self {
        ParseError::User {
            error: LexerError::UserError(error),
        }
    }
}
