mod ast;
mod lexer;
mod parser;

use ast::HotlistError;
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
        println!("{}", &curr_err);

        if let Some(p_err) = curr_err.downcast_ref::<ParseError<usize, lexer::Tok<'static>, LexerError<'static>>>() {
            if let Some(new_err) = parse_error_source(&p_err) {
                curr_err = new_err;
            } else {
                printing = false;
            }

            continue;
        }

        if let Some(new_err) = curr_err.source() {
            curr_err = new_err;
        } else {
            printing = false;
        }
    }

    std::process::exit(exit_code)
}

// ParseError does not provide a source() implementation, so I provide one here for the time
// being.
fn parse_error_source<'a>(p_err: &'a ParseError<usize, lexer::Tok<'static>, LexerError<'static>>) -> Option<&'a (dyn Error + 'static)> {
    match p_err {
        ParseError::User { error: ref u_err } => {
            Some(u_err)
        },
        _ => None
    }
}
