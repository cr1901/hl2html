use uuid::Uuid;
use url::Url;
use chrono::{DateTime, Utc};
use version_compare::version::Version;

use std::ops::Deref;
use std::convert::TryFrom;
use std::mem::replace;

pub struct HotList<'a> {
    pub version : Version<'a>,
    pub options : Options<'a>,
    pub entries : Vec<EntryKind>
}

pub struct Options<'a> {
    pub encoding: Encoding<'a>
}

// For now, assign default values to omitted options. Last value in repeated list takes priority.
pub enum SingleOp<'a> {
    Encoding(Encoding<'a>)
}

pub enum Encoding<'a> {
    Utf8(Version<'a>)
}

pub enum EntryKind {
    Folder(Folder),
    Notes(Vec<Note>)
}

pub struct Folder {
    pub id : u32,
    pub uuid : Uuid,
    pub name : String,
    pub timestamp : DateTime<Utc>,
    pub notes : Vec<Note>
}

pub struct Note {
    pub id : u32,
    pub uuid : Uuid,
    pub contents : String,
    pub url : Url
}
