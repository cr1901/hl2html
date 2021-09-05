use crate::ast::{HotlistError, SpanInfo};
use crate::lexer::{self, LexerError};
use crate::parser::*;

use argh::FromArgs;
use lalrpop_util::ParseError;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub fn print_error_and_exit<'a, T: AsRef<Path>>(
    err: Box<dyn Error + Send + Sync + 'a>,
    filename: T,
    exit_code: i32,
) -> ! {
    let mut printing = true;
    // Safety:
    // * Only place we want to downcast in the codebase.
    // * There is a single lifetime 'input tied to in_buf.
    // * "err"'s lifetime is tied to in_buf.
    // * Since we are using "err" at this point before any unsafe code, the lifetime must still
    //   be valid.
    // * No references escape this function- so we can't type pun non-static lifetimes as
    //   static lifetimes.
    let mut curr_err: &(dyn Error + 'static) = &*unsafe {
        std::mem::transmute::<&(dyn Error + Send + Sync + 'a), &(dyn Error + Send + Sync + 'static)>(
            &*err,
        )
    };

    while printing {
        if let Some(new_err) = curr_err.source() {
            curr_err = new_err;
        } else {
            println!("{}", &curr_err);
            printing = false;
        }
    }

    // Print location information about the error if possible.
    if let Some(p_err) =
        curr_err.downcast_ref::<ParseError<usize, lexer::Tok<'static>, LexerError<'static>>>()
    {
        let file = File::open(filename.as_ref()).unwrap_or_else(|e| {
            println!(
                "Could open {} to get error context: {}",
                filename.as_ref().display(),
                e
            );
            std::process::exit(exit_code);
        });
        let mut buf_reader = BufReader::new(file);

        match p_err {
            ParseError::UnrecognizedToken {
                token: (start, _, _end),
                expected: _,
            } => {
                let (context_str, (li_start, _)) = get_context(&mut buf_reader, *start, None)
                    .unwrap_or_else(|e| {
                        println!("Could not get error context: {}", e);
                        std::process::exit(exit_code);
                    });

                println!(
                    "unknown token begins at approximately line {}, offset {}\n{}",
                    li_start.line, li_start.offset, context_str
                );
            }
            ParseError::User {
                error: LexerError::UserError(hl_err),
            } => match hl_err {
                HotlistError::RequiredFieldMissing(_, SpanInfo { error: _, entry }) => {
                    let li_start =
                        get_line_and_offset(&mut buf_reader, entry.0).unwrap_or_else(|e| {
                            println!("Could not get error context: {}", e);
                            std::process::exit(exit_code);
                        });

                    let li_end =
                        get_line_and_offset(&mut buf_reader, entry.1).unwrap_or_else(|e| {
                            println!("Could not get error context: {}", e);
                            std::process::exit(exit_code);
                        });

                    buf_reader
                        .seek(SeekFrom::Start(entry.0 as u64))
                        .unwrap_or_else(|e| {
                            println!("Could not get error context: {}", e);
                            std::process::exit(exit_code);
                        });

                    let context_len = entry.1 - entry.0;
                    let mut vec = Vec::with_capacity(context_len);
                    buf_reader
                        .by_ref()
                        .take(context_len as u64)
                        .read_to_end(&mut vec)
                        .unwrap_or_else(|e| {
                            println!("Could not get error context: {}", e);
                            std::process::exit(exit_code);
                        });
                    let context_str = String::from_utf8_lossy(&vec);

                    println!(
                        "error begins on approximately line {}, offset {}\n{}",
                        li_start.line, li_start.offset, context_str
                    );
                }
                _ => {}
            },
            ParseError::User {
                error: LexerError::LexerError { char_idx: idx },
            } => {
                let (context_str, (li_start, _)) = get_context(&mut buf_reader, *idx, None)
                    .unwrap_or_else(|e| {
                        println!("Could not get error context: {}", e);
                        std::process::exit(exit_code);
                    });

                println!(
                    "unknown token begins at approximately line {}, offset {}\n{}",
                    li_start.line, li_start.offset, context_str
                );
            }
            _ => {}
        }
    }

    std::process::exit(exit_code);
}

fn get_context<R: Read + Seek>(
    buf_reader: &mut BufReader<R>,
    start: usize,
    _end: Option<i64>,
) -> Result<(String, (LineInfo, Option<LineInfo>)), Box<dyn Error + Send + Sync + 'static>> {
    let li_start = get_line_and_offset(buf_reader, start)?;
    buf_reader.seek(SeekFrom::Start(start as u64))?;

    let mut context_str = String::new();
    buf_reader.read_line(&mut context_str)?;
    Ok((context_str, (li_start, None)))
}
