mod ast;
mod lexer;
mod parser;

use ast::{HotlistError, SpanInfo};
use lexer::LexerError;
use parser::*;

use argh::FromArgs;
use lalrpop_util::ParseError;
use std::error::Error;

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
            print_error_and_exit(e, 1);
        }
    };
}

fn print_error_and_exit<'a>(err: Box<dyn Error + Send + Sync + 'a>, exit_code: i32) -> ! {
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

    while printing {
        // While walking the errors, handle specific err types specially.
        if let Some(p_err) = curr_err.downcast_ref::<ParseError<usize, lexer::Tok<'static>, LexerError<'static>>>() {
            if let Some(new_err) = parse_error_source(&p_err) {
                curr_err = new_err;
            } else {
                println!("{}", &curr_err);
                printing = false;
            }

            continue;
        } else if let Some(u_err) = curr_err.downcast_ref::<HotlistError<'static>>() {
            println!("{}", &u_err);
            match u_err {
                HotlistError::RequiredFieldMissing(_, SpanInfo { error, entry }) => {
                    unimplemented!()
                },
                _ => {}
            }

            printing = false;
            continue;
        }

        // Default actions: get next error in chain, if None, print, otherwise defer to next
        // level error.
        if let Some(new_err) = curr_err.source() {
            curr_err = new_err;
        } else {
            println!("{}", &curr_err);
            printing = false;
        }
    }

    std::process::exit(exit_code)
}

// ParseError does not provide a source() implementation, so I provide one in this crate. I don't
// think a from conversion from a newtype is possible here because I only have access to refs to
// ParseError, and the lifetime would become part of the newtype. This prevents me from converting
// back to Option<&'a (dyn Error + 'static)> without another horrific transmute.
//
// I will implement a ParseErrorWrapper newtype, if I figure out how to implement the conversion
// properly in the future.
fn parse_error_source<'a>(p_err: &'a ParseError<usize, lexer::Tok<'static>, LexerError<'static>>) -> Option<&'a (dyn Error + 'static)> {
    match p_err {
        ParseError::User { error: u_err } => {
            Some(u_err)
        },
        _ => None
    }
}
