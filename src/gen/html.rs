mod single;

use crate::ast::Hotlist;
use crate::error::Error;
use single::SingleEmitter;
use super::traverse_hotlist;

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

    if multi {
        return Err("multiple-file HTML rendering not implemented yet".into());
    } else {
        let mut emitter = SingleEmitter::new(out_handle);
        traverse_hotlist(hl, &mut emitter)?;
        let mut out_handle = emitter.into_inner();
        out_handle.flush()?;
    }

    Ok(())
}

trait HtmlEscapeWrite: Write {
    fn write_with_escapes(&mut self, raw: &str) -> io::Result<()> {
        let mut possible_newline = false;
        for c in raw.chars() {
            match c {
                '\x02' if !possible_newline => {
                    possible_newline = true;
                }
                '\x02' if possible_newline => {
                    write!(self, "</p>\n{:1$}<p>", " ", 4)?;
                    possible_newline = false;
                }
                '<' => {
                    write!(self, "&lt;")?;
                }
                '>' => {
                    write!(self, "&gt;")?;
                }
                '"' => {
                    write!(self, "&quot;")?;
                }
                '&' => {
                    write!(self, "&amp;")?;
                }
                '\'' => {
                    write!(self, "&apos;")?;
                }
                _ => {
                    write!(self, "{}", c)?;
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
}

impl<W> HtmlEscapeWrite for W where W: Write { }
