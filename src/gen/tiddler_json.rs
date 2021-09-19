mod single;

use super::traverse_hotlist;
use crate::ast::Hotlist;
use crate::error::Error;
use single::SingleGenerator;

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use serde::Serialize;
use serde_json::to_writer_pretty;

pub fn emit<T: AsRef<Path>>(
    filename: Option<T>,
    hl: &Hotlist,
    multi: bool,
) -> Result<(), Error<'static>> {
    if multi {
        // TODO: EmitError
        return Err("multiple-file output is not implemented for tiddlerjson".into());
    } else {
        let out_handle: Box<dyn Write> = if let Some(fn_) = filename {
            let file = File::create(fn_.as_ref())?;
            Box::new(BufWriter::new(file))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        };

        let mut gen = SingleGenerator::new();
        traverse_hotlist(hl, &mut gen)?;
        let mut emitter = to_writer_pretty(out_handle, &gen.into_inner())?;
    }

    Ok(())
}
