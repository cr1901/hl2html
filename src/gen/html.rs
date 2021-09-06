use super::{traverse_hotlist, Visitor};
use crate::ast::{EntryKind, Folder, Hotlist, Note};
use crate::error::Error;

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub fn emit<T: AsRef<Path>>(
    filename: Option<T>,
    hl: &Hotlist,
    multi: bool,
) -> Result<(), Error<'static>> {
    let out_handle: Box<dyn Write> = if let Some(fn_) = filename {
        let file = File::create(fn_.as_ref())?;
        Box::new(BufWriter::new(file))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    let mut emitter = HtmlEmitter::new(out_handle);
    traverse_hotlist(hl, &mut emitter)?;

    let mut out_handle = emitter.into_inner();
    out_handle.flush()?;

    Ok(())
}

struct HtmlEmitter<W>
where
    W: Write,
{
    buf: W,
}

impl<W> HtmlEmitter<W>
where
    W: Write,
{
    fn new(buf: W) -> Self {
        Self { buf }
    }

    fn write_with_escapes(&mut self, raw: &str) -> io::Result<()> {
        let mut possible_newline = false;
        for c in raw.chars() {
            match c {
                '\x02' if !possible_newline => {
                    possible_newline = true;
                }
                '\x02' if possible_newline => {
                    write!(self.buf, "</p>\n{:1$}<p>", " ", 4)?;
                    possible_newline = false;
                }
                '<' => {
                    write!(self.buf, "&lt;")?;
                }
                '>' => {
                    write!(self.buf, "&gt;")?;
                }
                '"' => {
                    write!(self.buf, "&quot;")?;
                }
                '&' => {
                    write!(self.buf, "&amp;")?;
                }
                '\'' => {
                    write!(self.buf, "&apos;")?;
                }
                _ => {
                    write!(self.buf, "{}", c)?;
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

    fn into_inner(self) -> W {
        self.buf
    }
}

impl<W> Visitor for HtmlEmitter<W>
where
    W: Write,
{
    fn visit_folder_empty(
        &mut self,
        f: &Folder,
    ) -> Result<(), Error<'static>> {
        write!(self.buf, "{:1$}<h2>Folder {2}</h2>\n", " ", 4, f.name)?;
        write!(self.buf, "{:1$}<ul>\n", " ", 4)?;
        write!(self.buf, "{:1$}<li>ID: {2}</li>\n", " ", 6, f.id)?;
        write!(self.buf, "{:1$}<li>UUID: {2}</li>\n", " ", 6, f.uuid)?;
        write!(self.buf, "{:1$}<li>Created: {2}</li>\n", " ", 6, f.timestamp)?;
        write!(self.buf, "{:1$}</ul>\n", " ", 4)?;

        write!(self.buf, "{:1$}<p>No Entries<p>\n", " ", 4)?;

        write!(self.buf, "\n")?;
        Ok(())
    }

    fn visit_folder_pre(
        &mut self,
        f: &Folder,
    ) -> Result<(), Error<'static>> {
        write!(self.buf, "{:1$}<h2>Folder {2}</h2>\n", " ", 4, f.name)?;
        write!(self.buf, "{:1$}<ul>\n", " ", 4)?;
        write!(self.buf, "{:1$}<li>ID: {2}</li>\n", " ", 6, f.id)?;
        write!(self.buf, "{:1$}<li>UUID: {2}</li>\n", " ", 6, f.uuid)?;
        write!(self.buf, "{:1$}<li>Created: {2}</li>\n", " ", 6, f.timestamp)?;
        write!(self.buf, "{:1$}</ul>\n", " ", 4)?;

        write!(self.buf, "\n")?;
        Ok(())
    }

    fn visit_folder_post(
        &mut self,
        f: &Folder,
    ) -> Result<(), Error<'static>> {
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
            write!(self.buf, "{:1$}<li>URL: <a href=\"{2}\">{2}</a></li>\n", " ", 6, u)?;
        } else {
            write!(self.buf, "{:1$}<li>URL: None</li>\n", " ", 6)?;
        }

        write!(self.buf, "{:1$}<li>Created: {2}</li>\n", " ", 6, n.timestamp)?;
        write!(self.buf, "{:1$}</ul>\n", " ", 4)?;

        if let Some(nbody) = n.contents {
            write!(self.buf, "{:1$}<p>", " ", 4)?;
            self.write_with_escapes(&nbody)?;
            write!(self.buf, "<p>\n")?;
        }

        write!(self.buf, "\n")?;
        Ok(())
    }

    fn visit_root_pre(
        &mut self,
        hl: &Hotlist,
    ) -> Result<(), Error<'static>> {
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

    fn visit_root_post(
        &mut self,
        _hl: &Hotlist,
    ) -> Result<(), Error<'static>> {
        write!(
            self.buf,
            r#"  </body>
</html>
"#
        )?;

        Ok(())
    }
}
