use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::collections::HashMap;
use std::path::PathBuf;

use bumpalo::Bump;
use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};

#[derive(Debug)]
enum SerializeType<'arena, 'input> {
    Input(&'input str),
    Arena(bumpalo::collections::String<'arena>),
    DateTime(super::DateTime),
    U32(u32),
    Uuid(super::Uuid)
}

impl<'arena, 'input> Serialize for SerializeType<'arena, 'input> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SerializeType::Input(s) => s.serialize(serializer),
            SerializeType::Arena(a) => a.serialize(serializer),
            SerializeType::DateTime(d) => d.serialize(serializer),
            SerializeType::U32(i) => i.serialize(serializer),
            SerializeType::Uuid(u) => u.serialize(serializer),
        }
    }
}

pub struct SingleGenerator<'arena, 'input> {
    json: Vec<HashMap<&'static str, SerializeType<'arena, 'input>>>,
    root: PathBuf,
    arena: &'arena Bump,
    now: DateTime<Utc>,
}

impl<'arena, 'input> Serialize for SingleGenerator<'arena, 'input> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.json.serialize(serializer)
    }
}

impl<'arena, 'input> SingleGenerator<'arena, 'input> {
    pub fn new(arena: &'arena Bump) -> Self {
        let json = Vec::<HashMap<&'static str, SerializeType>>::new();
        let root = PathBuf::new();
        let now = Utc::now();
        Self {
            json,
            root,
            arena,
            now,
        }
    }
}

impl<'a, 'arena, 'input> Visitor<'a, 'input> for SingleGenerator<'arena, 'input> {
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

        entry.insert("text", SerializeType::Input(n.contents.unwrap_or("")));

        // Hotlist-specific
        entry.insert("uuid", SerializeType::Uuid(n.uuid.into()));

        entry.insert("timestamp", SerializeType::DateTime(n.timestamp.into()));

        entry.insert("created", SerializeType::DateTime(self.now.into()));
        entry.insert("modified", SerializeType::DateTime(self.now.into()));
        entry.insert("tags", SerializeType::Input("opera"));

        let url = match &n.url {
            Some(u) => {
                bumpalo::format!(in &self.arena, "{}", u)
            }
            None => bumpalo::collections::String::from_str_in("None", &self.arena),
        };

        // TODO: When building the landing page, show URL for each entry but truncate to
        // a reasonable number of characters.
        let title = bumpalo::format!(in &self.arena, "Note {}", n.id);
        entry.insert("title", SerializeType::Arena(title));
        entry.insert("url", SerializeType::Arena(url));
        entry.insert("id", SerializeType::U32(n.id));

        let folder = bumpalo::format!(in &self.arena,
         "{}", self.root
                   .to_str()
                   .ok_or(format!("invalid path: {}",
                                  self.root.to_string_lossy()))?
        );
        entry.insert("folder", SerializeType::Arena(folder));
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
