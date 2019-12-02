use uuid::Uuid;
use url::Url;
use chrono::{DateTime, Utc};
use version_compare::version::Version;

pub struct HotList<'h> {
    version : Version<'h>,
    encoding : Encoding<'h>,
    entries : Vec<EntryKind<'h>>
}

pub enum Encoding<'e> {
    Utf8(Version<'e>)
}

pub enum EntryKind<'n> {
    Folder(Folder<'n>),
    Notes(Vec<Note<'n>>)
}

pub struct Folder<'f> {
    id : u32,
    uuid : Uuid,
    name : &'f str,
    timestamp : DateTime<Utc>,
    notes : Vec<Note<'f>>
}

pub struct Note<'n> {
    id : u32,
    uuid : Uuid,
    contents : &'n str,
    url : Url
}
