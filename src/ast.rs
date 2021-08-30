use uuid::Uuid;
use url::Url;
use chrono::{DateTime, Utc};
use version_compare::version::Version;

use std::ops::Deref;
use std::convert::TryFrom;
use std::mem::replace;

#[derive(Debug, PartialEq)]
pub struct HotList<'a> {
    pub version : Version<'a>,
    pub options : Options<'a>,
    pub entries : Vec<EntryKind>
}

#[derive(Debug, Default, PartialEq)]
pub struct Options<'a> {
    pub encoding: Encoding<'a>
}

// For now, assign default values to omitted options. Last value in repeated list takes priority.
#[derive(Debug, PartialEq)]
pub enum SingleOp<'a> {
    Encoding(Encoding<'a>)
}

#[derive(Debug, PartialEq)]
pub enum Encoding<'a> {
    Utf8(Version<'a>)
}

#[derive(Debug, PartialEq)]
pub enum EntryKind {
    Folder(Folder),
    Note(Note)
}

#[derive(Debug, PartialEq)]
pub struct Folder {
    pub id : u32,
    pub uuid : Uuid,
    pub name : String,
    pub timestamp : DateTime<Utc>,
    pub notes : Vec<Note>
}

#[derive(Debug, PartialEq)]
pub struct Note {
    pub id : u32,
    pub uuid : Uuid,
    pub contents : String,
    pub url : Url
}

impl<'a> Default for Encoding<'a> {
    fn default() -> Self {
        Encoding::Utf8(Version::from("0.0").unwrap())
    }
}
