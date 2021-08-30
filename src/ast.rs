use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;
use version_compare::version::Version;

#[derive(Debug, PartialEq)]
pub struct HotList<'a> {
    pub version: Version<'a>,
    pub options: Options<'a>,
    pub entries: Vec<EntryKind<'a>>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Options<'a> {
    pub encoding: Encoding<'a>,
}

// For now, assign default values to omitted options. Last value in repeated list takes priority.
#[derive(Debug, PartialEq)]
pub enum SingleOp<'a> {
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
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub notes: Vec<Note<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Note<'a> {
    pub id: u32,
    pub uuid: Uuid,
    pub contents: &'a str,
    pub url: Url,
    pub timestamp: DateTime<Utc>,
}

impl<'a> Default for Encoding<'a> {
    fn default() -> Self {
        Encoding::Utf8(Version::from("0.0").unwrap())
    }
}
