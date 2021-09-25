mod single;

use super::traverse_hotlist;
use crate::ast::Hotlist;
use crate::error::Error;
use single::SingleGenerator;

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use bumpalo::Bump;

use serde::Serialize;
use serde_json::ser;

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

        let bump = Bump::new();
        let mut gen = SingleGenerator::new(&bump);
        traverse_hotlist(hl, &mut gen)?;

        let formatter = TiddlerJsonEscapeWrite::new();
        let mut serializer = serde_json::Serializer::with_formatter(out_handle, formatter);

        gen.serialize(&mut serializer)?;
    }

    Ok(())
}

struct TiddlerJsonEscapeWrite<'a> {
    inner: ser::PrettyFormatter<'a>,
    possible_newline: bool,
}

impl<'a> TiddlerJsonEscapeWrite<'a> {
    fn new() -> Self {
        let inner = ser::PrettyFormatter::new();

        Self {
            inner,
            possible_newline: false,
        }
    }
}

// This is an unfortunate amount of boilerplate...
impl<'a> ser::Formatter for TiddlerJsonEscapeWrite<'a> {
    fn write_null<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_null(writer)
    }

    fn write_bool<W: ?Sized>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_bool(writer, value)
    }

    fn write_i8<W: ?Sized>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_i8(writer, value)
    }

    fn write_i16<W: ?Sized>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_i16(writer, value)
    }

    fn write_i32<W: ?Sized>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_i32(writer, value)
    }

    fn write_i64<W: ?Sized>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_i64(writer, value)
    }

    fn write_u8<W: ?Sized>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_u8(writer, value)
    }

    fn write_u16<W: ?Sized>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_u16(writer, value)
    }

    fn write_u32<W: ?Sized>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_u32(writer, value)
    }

    fn write_u64<W: ?Sized>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_u64(writer, value)
    }

    fn write_f32<W: ?Sized>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_f32(writer, value)
    }

    fn write_f64<W: ?Sized>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_f64(writer, value)
    }

    fn write_number_str<W: ?Sized>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_number_str(writer, value)
    }

    fn begin_string<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.begin_string(writer)
    }

    fn end_string<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.end_string(writer)
    }

    fn write_string_fragment<W: ?Sized>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: Write,
    {
        // We didn't get two control chars in a row, so write the buffered control char out and
        // keep going.
        if self.possible_newline {
            self.inner
                .write_char_escape(writer, ser::CharEscape::AsciiControl(0x02))?;
            self.possible_newline = false;
        }

        self.inner.write_string_fragment(writer, fragment)
    }

    fn write_char_escape<W: ?Sized>(
        &mut self,
        writer: &mut W,
        char_escape: ser::CharEscape,
    ) -> io::Result<()>
    where
        W: Write,
    {
        match char_escape {
            ser::CharEscape::AsciiControl(a) if a == 0x02 && self.possible_newline => {
                self.inner.write_string_fragment(writer, r"\n")?;
                self.possible_newline = false;
                Ok(())
            }
            ser::CharEscape::AsciiControl(a) if a == 0x02 && !self.possible_newline => {
                self.possible_newline = true;
                Ok(())
            }
            _ => self.inner.write_char_escape(writer, char_escape),
        }
    }

    fn begin_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.begin_array(writer)
    }

    fn end_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.end_array(writer)
    }

    fn begin_array_value<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.begin_array_value(writer, first)
    }

    fn end_array_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.end_array_value(writer)
    }

    fn begin_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.begin_object(writer)
    }

    fn end_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.end_object(writer)
    }

    fn begin_object_key<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.begin_object_key(writer, first)
    }

    fn end_object_key<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.end_object_key(writer)
    }

    fn begin_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.begin_object_value(writer)
    }

    fn end_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.end_object_value(writer)
    }

    fn write_raw_fragment<W: ?Sized>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: Write,
    {
        self.inner.write_raw_fragment(writer, fragment)
    }
}
