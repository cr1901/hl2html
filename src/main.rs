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
    #[argh(switch, short = 'm')]
    multiple: bool,
    /// emit hotlist in the selected mode (default HTML)
    #[argh(option, short = 'f', from_str_fn(output_format))]
    format: OutputFormat,
    /// output file or directory (if multiple files)
    #[argh(option, short = 'o')]
    output: Option<String>,
    /// input .adr file
    #[argh(positional)]
    path: String,
}

#[derive(PartialEq, Debug)]
enum OutputFormat {
    Html,
    Markdown,
    WikiText,
}

fn output_format(f: &str) -> Result<OutputFormat, String> {
    match f {
        "html" => Ok(OutputFormat::Html),
        "markdown" => Ok(OutputFormat::Markdown),
        "wikitext" => Ok(OutputFormat::WikiText),
        _ => Err(String::from(
            "unknown output format (html, markdown, wikitext)",
        )),
    }
}

fn main() {
    let args: HotlistArgs = argh::from_env();
    let mut in_buf = String::new();

    let hotlist = parser::parse_hotlist_from_file(&args.path, &mut in_buf).unwrap_or_else(|e| {
        println!("Error while parsing hotlist file:");
        error::print_error_and_exit(e, &args.path, 1);
    });

    match args.format {
        OutputFormat::Html => {
            gen::emit_hotlist_as_html((&args.output).as_ref(), &hotlist, args.multiple)
                .unwrap_or_else(|e| {
                    println!(
                        "Error while writing HTML file {}:",
                        &args.output.unwrap_or("to stdout".to_string())
                    );
                    error::print_error_and_exit(e, &args.path, 2);
                });
        }
        OutputFormat::Markdown => {
            println!(
                "Error while writing Markdown file {}:",
                &args.output.unwrap_or("to stdout".to_string())
            );
            error::print_error_and_exit(
                "markdown output not yet implemented".into(),
                &args.path,
                3,
            );
        }
        OutputFormat::WikiText => {
            println!(
                "Error while writing WikiText file {}:",
                &args.output.unwrap_or("to stdout".to_string())
            );
            error::print_error_and_exit(
                "wikitext output not yet implemented".into(),
                &args.path,
                4,
            );
        }
    }
}
