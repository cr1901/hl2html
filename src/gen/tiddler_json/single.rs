use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::collections::HashMap;
use std::io::Write;

pub struct SingleEmitter<'input, W>
where
    W: Write,
{
    buf: W,
    json: Vec<HashMap<&'static str, &'input str>>
}

impl<'input, W> SingleEmitter<'input, W>
where
    W: Write,
{
    pub fn new(buf: W) -> Self {
        let json = Vec::<HashMap::<&'static str, &'input str>>::new();
        Self { buf, json }
    }

    pub fn into_inner(self) -> W {
        self.buf
    }
}

impl<'a, 'input: 'a, W> Visitor<'a, 'input> for SingleEmitter<'input, W>
where
    W: Write,
{
    fn visit_folder_empty(&mut self, f: &'a Folder<'input>) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_folder_pre(&mut self, f: &'a Folder<'input>) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_folder_post(&mut self, f: &'a Folder<'input>) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_note(&mut self, n: &'a Note<'input>) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_root_pre(&mut self, hl: &'a Hotlist<'input>) -> Result<(), Error<'static>> {
        unimplemented!()
    }

    fn visit_root_post(&mut self, _hl: &'a Hotlist<'input>) -> Result<(), Error<'static>> {
        unimplemented!()
    }
}
