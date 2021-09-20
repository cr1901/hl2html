use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;


pub struct SingleGenerator<'a>
{
    json: Vec<HashMap<&'static str, Cow<'a, str>>>,
    root: PathBuf,
}

impl<'a> SingleGenerator<'a>
{
    pub fn new() -> Self {
        let json = Vec::<HashMap::<&'static str, Cow<'a, str>>>::new();
        let root = PathBuf::new();
        Self { json, root }
    }

    pub fn into_inner(self) -> Vec<HashMap<&'static str, Cow<'a, str>>> {
        self.json
    }
}

impl<'a, 'input: 'a> Visitor<'a, 'input> for SingleGenerator<'a>
{
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

        entry.insert("text", Cow::Borrowed(n.contents.unwrap_or("")));

        // Hotlist-specific
        entry.insert("uuid", Cow::Owned(n.uuid.to_string()));
        entry.insert("folder", Cow::Owned(self.root.to_str().unwrap_or("").to_owned()));

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
