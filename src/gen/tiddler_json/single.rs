use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};

#[derive(Debug)]
enum SerializeType<'input> {
    Str(&'input str),
    DateTime(super::DateTime),
    Folder(super::Folder),
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
            SerializeType::Str(s) => s.serialize(serializer),
            SerializeType::DateTime(d) => d.serialize(serializer),
            SerializeType::Folder(f) => f.serialize(serializer),
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

impl<'ast, 'input> Visitor<'ast, 'input> for SingleGenerator<'input> {
    fn visit_folder_empty(&mut self, _f: &'ast Folder<'input>) -> Result<(), Error<'static>> {
        Ok(())
    }

    fn visit_folder_pre(&mut self, f: &'ast Folder<'input>) -> Result<(), Error<'static>> {
        self.root.push(f.name);
        Ok(())
    }

    fn visit_folder_post(&mut self, _f: &'ast Folder<'input>) -> Result<(), Error<'static>> {
        self.root.pop();
        Ok(())
    }

    fn visit_note(&mut self, n: &'ast Note<'input>) -> Result<(), Error<'static>> {
        let mut entry = HashMap::new();

        entry.insert("text", SerializeType::NoteBody(n.contents.unwrap_or("").into()));

        // Hotlist-specific
        entry.insert("uuid", SerializeType::Uuid(n.uuid.into()));

        entry.insert("timestamp", SerializeType::DateTime(n.timestamp.into()));

        entry.insert("created", SerializeType::DateTime(self.now.into()));
        entry.insert("modified", SerializeType::DateTime(self.now.into()));
        entry.insert("tags", SerializeType::Str("opera"));

        // TODO: When building the landing page, show URL for each entry but truncate to
        // a reasonable number of characters.
        entry.insert("title", SerializeType::Title(n.id.into()));
        entry.insert("url", SerializeType::Url(n.url.clone().into()));
        entry.insert("id", SerializeType::U32(n.id));

        // TODO: Consider using elsa to avoid making many short-lived copies of the path root.
        entry.insert("folder", SerializeType::Folder(self.root.clone().into()));

        entry.insert("commit-sha", SerializeType::Str(env!("VERGEN_GIT_SHA")));

        self.json.push(entry);

        Ok(())
    }

    fn visit_root_pre(&mut self, _hl: &'ast Hotlist<'input>) -> Result<(), Error<'static>> {
        Ok(())
    }

    fn visit_root_post(&mut self, _hl: &'ast Hotlist<'input>) -> Result<(), Error<'static>> {
        // The final, main tiddler is the landing page.
        let mut entry = HashMap::new();

        entry.insert("title", SerializeType::Str("Opera Notes"));
        entry.insert(
            "text",
            SerializeType::Str(
                "<$list filter=\"[tag[opera]nsort[id]]\">\n<$link/> ({{!!url}})<br/>\n</$list>",
            ),
        );
        entry.insert("tags", SerializeType::Str("opera"));

        entry.insert("created", SerializeType::DateTime(self.now.into()));
        entry.insert("modified", SerializeType::DateTime(self.now.into()));

        entry.insert("commit-sha", SerializeType::Str(env!("VERGEN_GIT_SHA")));

        self.json.push(entry);

        Ok(())
    }
}
