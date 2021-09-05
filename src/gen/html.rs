use crate::ast::{Hotlist, EntryKind};

use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub fn emit<T: AsRef<Path>>(
    filename: Option<T>,
    hl: &Hotlist,
    multi: bool,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut out_handle: Box<dyn Write> = if let Some(fn_) = filename {
        let file = File::open(fn_.as_ref())?;
        Box::new(BufWriter::new(file))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    write!(
        out_handle,
        r#"<html>
  <head>
    <meta charset="utf-8">
    <title>Opera Hotlist</title>
  </head>
  <body>
"#
    )?;

    write!(
        out_handle,
        r#"<h1>{:1$}Opera Hotlist Version {2}</h1>
"#,
        " ",
        4,
        hl.version
    )?;

    let mut stack = Vec::<&EntryKind>::new();

    for e in hl.entries.iter().rev() {
        stack.push(e)
    }
    let mut last_visited: Option<&EntryKind> = None;

    loop {
        let curr = stack.last();

        if let None = curr {
            break;
        }

        let curr = curr.unwrap();

        match curr {
            EntryKind::Folder(f) => {

                if f.entries.len() != 0 && !nodes_equal(f.entries.last(), last_visited) {
                    write!(out_handle, "{:1$}<h2>Folder {2}</h2>\n", " ", 4, f.name)?;
                    for e in f.entries.iter().rev() {
                        stack.push(e);
                    }
                } else {
                    if f.entries.len() == 0 {
                        write!(out_handle, "{:1$}<h2>Folder {2}</h2>\n", " ", 4, f.name)?;
                        write!(out_handle, "{:1$}<p>No Entries<p>\n", " ", 4)?;
                    } else {
                        write!(out_handle, "{:1$}<p>End Folder {2}</p>\n", " ", 4, f.name)?;
                    }

                    last_visited = Some(curr);
                    stack.pop();
                }
            },
            EntryKind::Note(n) => {
                write!(out_handle, "{:1$}<h2>Note {2}</h2>\n", " ", 4, n.id)?;

                // without "&": cannot move out of `n.url.0` which is behind a shared reference
                if let Some(u) = &n.url {
                    write!(out_handle, r#"{:1$}<p>URL: <a href="{2}">{2}</a>
"#, " ", 4, u)?;
                } else {
                    write!(out_handle, "{:1$}<p>URL: None</p>\n", " ", 4)?;
                }

                if let Some(nbody) = n.contents {
                    write!(out_handle, "{:1$}<p>", " ", 4)?;

                    let mut possible_newline = false;
                    for c in nbody.chars() {
                        if c == '\x02' && !possible_newline {
                            possible_newline = true;
                        } else if c == '\x02' && possible_newline {
                            write!(out_handle, "</p>\n{:1$}<p>", " ", 4)?;
                            possible_newline = false;
                        } else if c == '<' {
                            write!(out_handle, "&lt;")?;
                        } else if c == '>' {
                            write!(out_handle, "&gt;")?;
                        } else {
                            write!(out_handle, "{}", c)?;
                        }
                    }
                    write!(out_handle, "<p>\n")?;
                }

                last_visited = Some(curr);
                stack.pop();
            }
        }
    }

    write!(
        out_handle,
        r#"  </body>
</html>"#
    )?;

    out_handle.flush()?;

    Ok(())
}

fn nodes_equal<'a>(a: Option<&EntryKind<'a>>, b: Option<&EntryKind<'a>>) -> bool {
     if a.is_none() || b.is_none() {
         return false;
     } else {
         let a_ref = a.unwrap();
         let b_ref = b.unwrap();

         return std::ptr::eq(a_ref, b_ref);
     }
 }
