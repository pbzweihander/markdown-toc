extern crate getopts;
extern crate markdown_toc;

use markdown_toc::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

fn parse_command(opts: &mut getopts::Options, args: &[String]) -> Result<Config, ()> {
    opts.optflag("h", "help", "print this help message")
        .optopt(
            "",
            "bullet",
            "Custom bullet of the ToC list. (default: \"1.\")",
            "{str}",
        )
        .optopt(
            "",
            "indent",
            "Indentation of the ToC list. (default: 4)",
            "{int}",
        )
        .optopt("", "max-depth", "Max depth of headers to include.", "{int}")
        .optopt(
            "",
            "min-depth",
            "Min depth of headers to include. (default: 0)",
            "{int}",
        )
        .optopt(
            "",
            "header",
            "Custom header of the ToC. (default: \"## Table of Contents\")",
            "{str}",
        )
        .optflag("", "no-link", "Exclude links in ToC")
        .optflag("", "no-header", "Exclude the header of ToC")
        .optflag(
            "i",
            "inline",
            "With this flag, the full markdown file will be printed with ToC."
        )
        .optflag(
            "r",
            "replace", "Should be used with --inline option and FILE should not be stdin. The original file will be replace instead of printing to standard output."
        );

    let opt_matches = opts.parse(args).map_err(|_| ())?;

    if opt_matches.opt_present("h") {
        return Err(());
    }

    let default = Config::default();

    macro_rules! get_opt_or_default {
        ($o:expr, $p:ident) => {
            if let Some(val) = opt_matches.opt_get($o).map_err(|_| ())? {
                val
            } else {
                default.$p
            }
        };
    }
    macro_rules! get_opt_or_default_option {
        ($o:expr, $p:ident) => {
            opt_matches.opt_get($o).map_err(|_| ())?.or(default.$p)
        };
    }

    let (input_file, is_input_stdin) = if !opt_matches.free.is_empty() {
        if opt_matches.free[0] == "-" {
            (InputFile::StdIn, true)
        } else {
            (InputFile::Path(PathBuf::from(&opt_matches.free[0])), false)
        }
    } else {
        return Err(());
    };

    Ok(Config {
        input_file: if !opt_matches.free.is_empty() {
            if opt_matches.free[0] == "-" {
                InputFile::StdIn
            } else {
                InputFile::Path(PathBuf::from(&opt_matches.free[0]))
            }
        } else {
            return Err(());
        },
        bullet: get_opt_or_default!("bullet", bullet),
        indent: get_opt_or_default!("indent", indent),
        max_depth: get_opt_or_default_option!("max-depth", max_depth),
        min_depth: get_opt_or_default!("min-depth", min_depth),
        no_link: opt_matches.opt_present("no-link"),
        header: if opt_matches.opt_present("no-header") {
            None
        } else {
            get_opt_or_default_option!("header", header)
        },
        inline: match (
            opt_matches.opt_present("inline"),
            opt_matches.opt_present("replace"),
            is_input_stdin,
        ) {
            (true, true, false) => Inline::InlineAndReplace,
            (true, false, _) => Inline::Inline,
            (false, false, _) => Inline::None,
            _ => return Err(()),
        },
    })
}

fn print_help(program: &str, opts: &getopts::Options) {
    let brief = format!("Usage: {} FILE [options]\n\n   FILE        The Markdown file to parse for table of contents,\n               or \"-\" to read from stdin", program);
    eprint!("{}", opts.usage(&brief));
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    let config = match parse_command(&mut opts, &args[1..]) {
        Ok(c) => c,
        Err(_) => {
            print_help(&program, &opts);
            return;
        }
    };

    let mut content = String::new();
    match config.input_file {
        InputFile::StdIn => std::io::stdin().read_to_string(&mut content),
        InputFile::Path(ref p) => File::open(p).unwrap().read_to_string(&mut content),
    }
    .unwrap();

    println!("");

    if let Some(ref header) = config.header {
        println!("{}\n", header);
    }

    let mut code_fence = Fence::None;

    let headings = content
        .lines()
        .filter(|line| match code_fence {
            Fence::None => {
                if line.starts_with("```") || line.starts_with("~~~") {
                    code_fence = Fence::Open(&line[..3]);
                    false
                } else {
                    true
                }
            }
            Fence::Open(tag) => {
                if line.starts_with(tag) {
                    code_fence = Fence::None;
                }
                false
            }
        })
        .map(Heading::from_str)
        .filter_map(Result::ok)
        .filter_map(|h| h.format(&config)).collect::<Vec<String>>();

    let print_toc = || {
        headings.iter().for_each(|h| println!("{}", h));
    };

    match config.inline {
        Inline::Inline => {
            print_toc();
            println!("{}", content)
        },
        Inline::InlineAndReplace => {},
        _ => {
            print_toc()
        }
    }
}

enum Fence<'e> {
    Open(&'e str),
    None,
}
