mod ast;
mod lexer;
mod parser;

use ast::{HotlistError, SpanInfo};
use lexer::LexerError;
use parser::*;

use argh::FromArgs;
use lalrpop_util::ParseError;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(FromArgs, PartialEq, Debug)]
/// A command with positional arguments.
struct HotlistArgs {
    #[argh(positional)]
    path: String,
}

fn main() {
    let args: HotlistArgs = argh::from_env();
    let mut in_buf = String::new();

    let hotlist = match parse_hotlist_from_file(&args.path, &mut in_buf) {
        Ok(hl) => hl,
        Err(e) => {
            println!("Error while parsing hotlist file {}:", args.path);
            print_error_and_exit(e, &args.path, 1);
        }
    };
}

fn print_error_and_exit<'a, T: AsRef<Path>>(err: Box<dyn Error + Send + Sync + 'a>, filename: T, exit_code: i32) -> ! {
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
        std::mem::transmute::<&(dyn Error + Send + Sync + 'a),
                              &(dyn Error + Send + Sync + 'static)>(&*err)
    };

    let orig_err = curr_err.clone();

    while printing {
        if let Some(new_err) = curr_err.source() {
            curr_err = new_err;
        } else {
            println!("{}", &curr_err);
            printing = false;
        }
    }

    // Print location information about the error if possible.
    if let Some(p_err) = curr_err.downcast_ref::<ParseError<usize, lexer::Tok<'static>, LexerError<'static>>>() {
        let file = File::open(filename.as_ref()).unwrap_or_else(
            |e| {
                println!("Could open {} to get error context: {}", filename.as_ref().display(), e);
                std::process::exit(exit_code);
            }
        );
        let mut buf_reader = BufReader::new(file);

        match p_err {
            ParseError::User { error: LexerError::UserError(hl_err) } => {
                match hl_err {
                    HotlistError::RequiredFieldMissing(_, SpanInfo { error, entry }) => {
                        let li_start = get_line_and_offset(buf_reader, entry.0).unwrap_or_else(
                            |e| {
                                println!("Could not get error context: {}", e);
                                std::process::exit(exit_code);
                            }
                        );

                        println!("error begins on approximately line {}, offset {}", li_start.line, li_start.offset);
                    },
                    _ => { }
                }
            },
            ParseError::User { error: LexerError::LexerError { char_idx: _ } } => {
                unimplemented!()
            },
            _ => { }
        }
    }

    std::process::exit(exit_code);
}
