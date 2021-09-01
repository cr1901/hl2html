mod ast;
mod lexer;
mod parser;

use ast::HotlistError;
use lexer::LexerError;
use parser::parse_hotlist_from_file;

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
    let mut curr_err = err;

    while printing {
        println!("{}", curr_err);
        printing = false;
    }

    std::process::exit(exit_code)
}
