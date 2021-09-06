use super::HtmlEscapeWrite;
use crate::ast::{Folder, Hotlist, Note};
use crate::error::Error;
use crate::gen::Visitor;

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{PathBuf};

pub struct MultiEmitter {
    root: PathBuf,
}

impl MultiEmitter {
    pub fn new<P>(root: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { root: root.into() }
    }

    fn write_note(&mut self, n: &Note) -> Result<(), Error<'static>> {
        self.root.push(n.id.to_string());
        self.root.set_extension("html");

        // It is assumed that the entire directory is recreated each time. No effort is made
        // to restart an interrupted file generation. This check is here in case IDs are, in
        // fact, not unique in practice.
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.root)?;

        let mut buf = BufWriter::new(file);

        write!(
            buf,
            r#"<html>
  <head>
    <meta charset="utf-8">
    <title>Opera Hotlist: Note {}</title>
  </head>
  <body>
"#,
            n.id
        )?;

        write!(
            buf,
            r#"    <h1>Note {0}</h1>
    <ul>
      <li>UUID: {1}</li>
"#,
            n.id, n.uuid
        )?;

        // without "&": cannot move out of `n.url.0` which is behind a shared reference
        if let Some(u) = &n.url {
            write!(
                buf,
                r#"      <li>URL: <a href="{0}">{0}</a></li>
"#,
                u
            )?;
        } else {
            write!(
                buf,
                r#"      <li>URL: None</li>
"#
            )?;
        }

        write!(
            buf,
            r#"      <li>Created: {}</li>
    </ul>
"#,
            n.timestamp
        )?;

        if let Some(nbody) = n.contents {
            write!(buf, "    <p>")?;
            buf.write_with_escapes(&nbody)?;
            write!(buf, "<p>\n")?;
        }

        write!(
            buf,
            r#"  </body>
</html>
"#
        )?;

        buf.flush()?;
        self.root.pop();
        Ok(())
    }

    fn write_folder_meta(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        self.root.push("meta.txt");

        let mut file = File::create(&self.root)?;
        write!(
            file,
            r"Name: {}
ID: {}
UUID: {}
Created: {}
Expanded: {}
Trash: {}
Number of Entries: {}
",
            f.name,
            f.id,
            f.uuid,
            f.timestamp,
            f.expanded,
            f.trash,
            f.entries.len()
        )?;

        self.root.pop();
        Ok(())
    }

    fn write_root_meta(&mut self, h: &Hotlist) -> Result<(), Error<'static>> {
        self.root.push("meta.txt");

        let mut file = File::create(&self.root)?;
        write!(
            file,
            r"Hotlist Root
Number of Entries: {}
",
            h.entries.len()
        )?;

        self.root.pop();
        Ok(())
    }
}

impl Visitor for MultiEmitter {
    fn visit_folder_empty(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        self.root.push(f.name);
        create_dir_all(&self.root)?;

        self.write_folder_meta(f)?;

        self.root.pop();
        Ok(())
    }
    fn visit_folder_pre(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        self.root.push(f.name);
        create_dir_all(&self.root)?;
        Ok(())
    }
    fn visit_folder_post(&mut self, f: &Folder) -> Result<(), Error<'static>> {
        self.write_folder_meta(f)?;

        self.root.pop();
        Ok(())
    }
    fn visit_note(&mut self, n: &Note) -> Result<(), Error<'static>> {
        self.write_note(n)?;
        Ok(())
    }
    fn visit_root_pre(&mut self, _hotlist: &Hotlist) -> Result<(), Error<'static>> {
        create_dir_all(&self.root)?;
        Ok(())
    }
    fn visit_root_post(&mut self, h: &Hotlist) -> Result<(), Error<'static>> {
        self.write_root_meta(h)?;
        Ok(())
    }
}
