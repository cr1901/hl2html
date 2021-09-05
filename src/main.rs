mod ast;
mod error;
mod gen;
mod lexer;
mod parser;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Convert Opera Hotlist files to another format
struct HotlistArgs {
    /// create multiple files, index is main file.
    #[argh(switch, short='m')]
    multiple: bool,
    /// emit hotlist in the selected mode (default HTML)
    #[argh(option, short='f', from_str_fn(output_format))]
    format: OutputFormat,
    /// output file or directory (if multiple files)
    #[argh(option, short='o')]
    output: Option<String>,
    /// input .adr file
    #[argh(positional)]
    path: String,
}

#[derive(PartialEq, Debug)]
enum OutputFormat {
    Html,
    Markdown,
    WikiText
}

fn output_format(f: &str) -> Result<OutputFormat, String> {
    match f {
        "html" => Ok(OutputFormat::Html),
        "markdown" => Ok(OutputFormat::Markdown),
        "wikitext" => Ok(OutputFormat::WikiText),
        _ => Err(String::from("unknown output format (html, markdown, wikitext)")),
    }
}

fn main() {
    let args: HotlistArgs = argh::from_env();
    let mut in_buf = String::new();

    let hotlist = match parser::parse_hotlist_from_file(&args.path, &mut in_buf) {
        Ok(hl) => hl,
        Err(e) => {
            println!("Error while parsing hotlist file(s)");
            error::print_error_and_exit(e, &args.path, 1);
        }
    };

    match args.format {
        OutputFormat::Html => {
            match gen::emit_hotlist_as_html(args.output, &hotlist, args.multiple) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error while writing HTML file {}:", args.path);
                    error::print_error_and_exit(e, &args.path, 2);
                }
            }
        },
        OutputFormat::Markdown => {
            println!("markdown output not yet implemented");
            std::process::exit(3);
        },
        OutputFormat::WikiText => {
            println!("wikitext output not yet implemented");
            std::process::exit(3);
        }
    }
}
