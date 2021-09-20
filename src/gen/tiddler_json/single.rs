use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use bumpalo::Bump;
use serde::{Serialize, Serializer};

#[derive(Debug)]
enum StringRefs<'arena, 'input> {
    Input(&'input str),
    Arena(bumpalo::collections::String<'arena>),
}

impl<'arena, 'input> Serialize for StringRefs<'arena, 'input> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            StringRefs::Input(s) => s.serialize(serializer),
            StringRefs::Arena(a) => a.serialize(serializer)
        }
    }
}

pub struct SingleGenerator<'arena, 'input> {
    json: Vec<HashMap<&'static str, StringRefs<'arena, 'input>>>,
    root: PathBuf,
    arena: &'arena Bump
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
        let json = Vec::<HashMap<&'static str, StringRefs>>::new();
        let root = PathBuf::new();
        Self { json, root, arena }
    }
}

impl<'a, 'arena, 'input: 'a> Visitor<'a, 'input> for SingleGenerator<'arena, 'input> {
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

        entry.insert("text", StringRefs::Input(n.contents.unwrap_or("")));

        // Hotlist-specific
        let uuid = bumpalo::format!(in &self.arena, "{}", n.uuid.to_hyphenated_ref());
        entry.insert("uuid", StringRefs::Arena(uuid));

        // let folder = bumpalo::format!(in &self.arena, "{}", self.root);

        // let
        // entry.insert(
        //     "folder",
        //     Cow::Owned(self.root.to_string_lossy().to_string()),
        // );

        self.json.push(entry);

        Ok(())
    }

    fn visit_root_pre(&mut self, _hl: &'a Hotlist<'input>) -> Result<(), Error<'static>> {
        Ok(())
    }

    fn visit_root_post(&mut self, _hl: &'a Hotlist<'input>) -> Result<(), Error<'static>> {
        Ok(())
    }
}
