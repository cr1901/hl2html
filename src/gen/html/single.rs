use super::HtmlEscapeWrite;
use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::io::{self, Write};

pub struct SingleEmitter<W>
where
    W: Write,
{
    buf: W,
}

impl<W> SingleEmitter<W>
where
    W: Write,
{
    pub fn new(buf: W) -> Self {
        Self { buf }
    }

    pub fn into_inner(self) -> W {
        self.buf
    }
}

impl<W> Visitor for SingleEmitter<W>
where
    W: Write,
{
    fn visit_folder_empty(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        write!(self.buf, "{:1$}<h2>Folder {2}</h2>\n", " ", 4, f.name)?;
        write!(self.buf, "{:1$}<ul>\n", " ", 4)?;
        write!(self.buf, "{:1$}<li>ID: {2}</li>\n", " ", 6, f.id)?;
        write!(self.buf, "{:1$}<li>UUID: {2}</li>\n", " ", 6, f.uuid)?;
        write!(
            self.buf,
            "{:1$}<li>Created: {2}</li>\n",
            " ", 6, f.timestamp
        )?;
        write!(self.buf, "{:1$}</ul>\n", " ", 4)?;

        write!(self.buf, "{:1$}<p>No Entries<p>\n", " ", 4)?;

        write!(self.buf, "\n")?;
        Ok(())
    }

    fn visit_folder_pre(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        write!(self.buf, "{:1$}<h2>Folder {2}</h2>\n", " ", 4, f.name)?;
        write!(self.buf, "{:1$}<ul>\n", " ", 4)?;
        write!(self.buf, "{:1$}<li>ID: {2}</li>\n", " ", 6, f.id)?;
        write!(self.buf, "{:1$}<li>UUID: {2}</li>\n", " ", 6, f.uuid)?;
        write!(
            self.buf,
            "{:1$}<li>Created: {2}</li>\n",
            " ", 6, f.timestamp
        )?;
        write!(self.buf, "{:1$}</ul>\n", " ", 4)?;

        write!(self.buf, "\n")?;
        Ok(())
    }

    fn visit_folder_post(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        write!(self.buf, "{:1$}<p>End Folder {2}</p>\n", " ", 4, f.name)?;
        write!(self.buf, "\n")?;
        Ok(())
    }

    fn visit_note(&mut self, n: &Note) -> Result<(), Error<'static>> {
        write!(self.buf, "{:1$}<h2>Note {2}</h2>\n", " ", 4, n.id)?;
        write!(self.buf, "{:1$}<ul>\n", " ", 4)?;
        write!(self.buf, "{:1$}<li>UUID: {2}</li>\n", " ", 6, n.uuid)?;

        // without "&": cannot move out of `n.url.0` which is behind a shared reference
        if let Some(u) = &n.url {
            write!(
                self.buf,
                "{:1$}<li>URL: <a href=\"{2}\">{2}</a></li>\n",
                " ", 6, u
            )?;
        } else {
            write!(self.buf, "{:1$}<li>URL: None</li>\n", " ", 6)?;
        }

        write!(
            self.buf,
            "{:1$}<li>Created: {2}</li>\n",
            " ", 6, n.timestamp
        )?;
        write!(self.buf, "{:1$}</ul>\n", " ", 4)?;

        if let Some(nbody) = n.contents {
            write!(self.buf, "{:1$}<p>", " ", 4)?;
            self.buf.write_with_escapes(&nbody)?;
            write!(self.buf, "<p>\n")?;
        }

        write!(self.buf, "\n")?;
        Ok(())
    }

    fn visit_root_pre(&mut self, hl: &Hotlist) -> Result<(), Error<'static>> {
        write!(
            self.buf,
            r#"<html>
  <head>
    <meta charset="utf-8">
    <title>Opera Hotlist</title>
  </head>
  <body>
"#
        )?;

        write!(
            self.buf,
            "{:1$}<h1>Opera Hotlist Version {2}</h1>\n",
            " ", 4, hl.version
        )?;

        Ok(())
    }

    fn visit_root_post(&mut self, _hl: &Hotlist) -> Result<(), Error<'static>> {
        write!(
            self.buf,
            r#"  </body>
</html>
"#
        )?;

        Ok(())
    }
}
