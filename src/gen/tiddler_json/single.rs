use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};

#[derive(Debug)]
enum SerializeType<'input> {
    Input(&'input str),
    DateTime(super::DateTime),
    NoteBody(super::NoteBody<'input>),
    Title(super::Title),
    U32(u32),
    Url(super::Url),
    Uuid(super::Uuid)
}

impl<'input> Serialize for SerializeType<'input> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SerializeType::Input(s) => s.serialize(serializer),
            SerializeType::DateTime(d) => d.serialize(serializer),
            SerializeType::NoteBody(n) => n.serialize(serializer),
            SerializeType::Title(t) => t.serialize(serializer),
            SerializeType::U32(i) => i.serialize(serializer),
            SerializeType::Url(u) => u.serialize(serializer),
            SerializeType::Uuid(u) => u.serialize(serializer),
        }
    }
}

pub struct SingleGenerator<'input> {
    json: Vec<HashMap<&'static str, SerializeType<'input>>>,
    root: PathBuf,
    now: DateTime<Utc>,
}

impl<'input> Serialize for SingleGenerator<'input> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.json.serialize(serializer)
    }
}

impl<'input> SingleGenerator<'input> {
    pub fn new() -> Self {
        let json = Vec::<HashMap<&'static str, SerializeType>>::new();
        let root = PathBuf::new();
        let now = Utc::now();
        Self {
            json,
            root,
            now,
        }
    }
}

impl<'a, 'input> Visitor<'a, 'input> for SingleGenerator<'input> {
    fn visit_folder_empty(&mut self, _f: &'a Folder<'input>) -> Result<(), Error<'static>> {
        Ok(())
    }

    fn visit_folder_pre(&mut self, f: &'a Folder<'input>) -> Result<(), Error<'static>> {
        self.root.push(f.name);
        Ok(())
    }

    fn visit_folder_post(&mut self, _f: &'a Folder<'input>) -> Result<(), Error<'static>> {
        self.root.pop();
        Ok(())
    }

    fn visit_note(&mut self, n: &'a Note<'input>) -> Result<(), Error<'static>> {
        let mut entry = HashMap::new();

        entry.insert("text", SerializeType::NoteBody(n.contents.unwrap_or("").into()));

        // Hotlist-specific
        entry.insert("uuid", SerializeType::Uuid(n.uuid.into()));

        entry.insert("timestamp", SerializeType::DateTime(n.timestamp.into()));

        entry.insert("created", SerializeType::DateTime(self.now.into()));
        entry.insert("modified", SerializeType::DateTime(self.now.into()));
        entry.insert("tags", SerializeType::Input("opera"));

        // TODO: When building the landing page, show URL for each entry but truncate to
        // a reasonable number of characters.
        entry.insert("title", SerializeType::Title(n.id.into()));
        entry.insert("url", SerializeType::Url(n.url.clone().into()));
        entry.insert("id", SerializeType::U32(n.id));

        entry.insert("commit-sha", SerializeType::Input(env!("VERGEN_GIT_SHA")));

        self.json.push(entry);

        Ok(())
    }

    fn visit_root_pre(&mut self, _hl: &'a Hotlist<'input>) -> Result<(), Error<'static>> {
        Ok(())
    }

    fn visit_root_post(&mut self, _hl: &'a Hotlist<'input>) -> Result<(), Error<'static>> {
        // The final, main tiddler is the landing page.
        let mut entry = HashMap::new();

        entry.insert("title", SerializeType::Input("Opera Notes"));
        entry.insert(
            "text",
            SerializeType::Input(
                "<$list filter=\"[tag[opera]nsort[id]]\">\n<$link/> ({{!!url}})<br/>\n</$list>",
            ),
        );
        entry.insert("tags", SerializeType::Input("opera"));

        entry.insert("created", SerializeType::DateTime(self.now.into()));
        entry.insert("modified", SerializeType::DateTime(self.now.into()));

        entry.insert("commit-sha", SerializeType::Input(env!("VERGEN_GIT_SHA")));

        self.json.push(entry);

        Ok(())
    }
}
