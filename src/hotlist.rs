use uuid::Uuid;
use url::Url;
use chrono::{DateTime, Utc};
use version_compare::version::Version;

use std::ops::Deref;
use std::convert::TryFrom;
use std::mem::replace;

pub struct HotList<'a> {
    pub version : Version<'a>,
    pub encoding : Encoding<'a>,
    pub entries : Vec<EntryKind>
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
