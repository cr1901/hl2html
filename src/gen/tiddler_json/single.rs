use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::collections::HashMap;
use std::io::Write;

pub struct SingleEmitter<'a, W>
where
    W: Write,
{
    buf: W,
    json: Vec<HashMap<&'static str, &'a str>>
}

impl<'a, W> SingleEmitter<'a, W>
where
    W: Write,
{
    pub fn new(buf: W) -> Self {
        let json = Vec::<HashMap::<&'static str, &'a str>>::new();
        Self { buf, json }
    }

    pub fn into_inner(self) -> W {
        self.buf
    }
}

impl<'a, W> Visitor for SingleEmitter<'a, W>
where
    W: Write,
{
    fn visit_folder_empty(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_folder_pre(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_folder_post(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_note(&mut self, n: &Note) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_root_pre(&mut self, hl: &Hotlist) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_root_post(&mut self, _hl: &Hotlist) -> Result<(), Error<'static>> {
        unimplemented!()
    }
}
