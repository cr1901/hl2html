use uuid::Uuid;
use url::Url;
use chrono::{DateTime, Utc};
use version_compare::version::Version as RefVersion;

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


pub struct Version<'a> {
    str: String,
    version: RefVersion<'a>
}

impl TryFrom<String> for Version<'_> {
    type Error = &'static str;

    fn try_from(s : String) -> Result<Self, Self::Error> {

        let mut ver = Version {
            str : s,
            version : RefVersion::from("0.0").unwrap()
        };

        let r = {
            let ref_ver = RefVersion::from(&ver.str);

            if ref_ver.is_none() {
                return Err("Version parse failure.")
            }

            ref_ver.unwrap()
        };

        replace(&mut ver.version, r);
        Ok(ver)
    }
}

impl<'a> Deref for Version<'a> {
    type Target = RefVersion<'a>;

    fn deref(&self) -> &Self::Target {
        &self.version
    }
}
