mod single;

use super::traverse_hotlist;
use crate::ast::Hotlist;
use crate::error::Error;
use single::SingleGenerator;

use std::fmt;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use chrono::{self, Utc};
use uuid;
use url;

use serde::{Serialize, Serializer};
use serde_json::ser;

pub fn emit<T: AsRef<Path>>(
    filename: Option<T>,
    hl: &Hotlist,
    multi: bool,
) -> Result<(), Error<'static>> {
    if multi {
        // TODO: EmitError
        return Err("multiple-file output is not implemented for tiddlerjson".into());
    } else {
        let out_handle: Box<dyn Write> = if let Some(fn_) = filename {
            let file = File::create(fn_.as_ref())?;
            Box::new(BufWriter::new(file))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        };

        let mut gen = SingleGenerator::new();
        traverse_hotlist(hl, &mut gen)?;

        let mut serializer = serde_json::Serializer::pretty(out_handle);

        gen.serialize(&mut serializer)?;
    }

    Ok(())
}

// TODO: Nominally, I don't want Serialize to be implemented for these types when a Serializer's
// collect_str implementation heap-allocates, but I'm not sure how to constrain to "only implement
// for specific serializers" at this time.
#[derive(Debug)]
struct Uuid(uuid::Uuid);

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0.to_hyphenated_ref())
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Uuid(uuid)
    }
}

#[derive(Debug)]
struct DateTime(chrono::DateTime<Utc>);

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0.format("%Y%m%d%H%M%S%3f"))
    }
}

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(datetime: chrono::DateTime<Utc>) -> Self {
        DateTime(datetime)
    }
}

#[derive(Debug)]
struct NoteBody<'a>(&'a str);

impl<'a> fmt::Display for NoteBody<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut possible_newline = false;
        for c in self.0.chars() {
            match c {
                '\x02' if !possible_newline => {
                    possible_newline = true;
                }
                '\x02' if possible_newline => {
                    write!(f, "\n")?;
                    possible_newline = false;
                }
                '<' => {
                    write!(f, "&lt;")?;
                }
                '>' => {
                    write!(f, "&gt;")?;
                }
                '"' => {
                    write!(f, "&quot;")?;
                }
                '&' => {
                    write!(f, "&amp;")?;
                }
                '\'' => {
                    write!(f, "&apos;")?;
                }
                _ => {
                    write!(f, "{}", c)?;
                }
            }

            // We're only interested in matching two \x02 chars back-to-back.
            match c {
                '\x02' => {}
                _ => {
                    possible_newline = false;
                }
            }
        }

        Ok(())
    }
}

impl<'a> Serialize for NoteBody<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format_args!("{}", &self))
    }
}

impl<'a> From<&'a str> for NoteBody<'a> {
    fn from(body: &'a str) -> Self {
        NoteBody(body)
    }
}

#[derive(Debug)]
struct Folder(PathBuf);

impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_str().ok_or(fmt::Error)?)
    }
}

impl Serialize for Folder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format_args!("{}", &self))
    }
}

impl From<PathBuf> for Folder {
    fn from(pathbuf: PathBuf) -> Self {
        Folder(pathbuf)
    }
}

#[derive(Debug)]
struct Url(Option<url::Url>);

impl<'a> Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Some(u) => serializer.collect_str(&format_args!("{}", u)),
            None => serializer.serialize_str("None")
        }
    }
}

impl From<Option<url::Url>> for Url {
    fn from(url: Option<url::Url>) -> Self {
        Url(url)
    }
}

#[derive(Debug)]
struct Title(u32);

impl Serialize for Title {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format_args!("Note {}", self.0))
    }
}

impl From<u32> for Title {
    fn from(id: u32) -> Self {
        Title(id)
    }
}
