extern crate getopts;
#[macro_use]
extern crate slugify;

use slugify::slugify;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

struct Heading {
    depth: usize,
    title: String,
}

impl FromStr for Heading {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.starts_with("#") {
            let mut depth = 0usize;
            let title = trimmed
                .chars()
                .skip_while(|c| {
                    if *c == '#' {
                        depth += 1;
                        true
                    } else {
                        false
                    }
                })
                .collect::<String>()
                .trim_left()
                .to_owned();
            Ok(Heading {
                depth: depth - 1,
                title,
            })
        } else {
            Err(())
        }
    }
}

enum InputFile {
    Path(PathBuf),
    StdIn,
}

// enum Inline {
//     None,
//     Inline,
//     InlineAndReplace,
// }

struct Config {
    input_file: InputFile,
    bullet: String,
    indent: usize,
    max_depth: Option<usize>,
    min_depth: usize,
    header: Option<String>,
    no_link: bool,
    // inline: Inline,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            input_file: InputFile::StdIn,
            bullet: String::from("1."),
            indent: 4,
            max_depth: None,
            min_depth: 0,
            no_link: false,
            header: Some(String::from("## Table of Contents")),
            // inline: Inline::None,
        }
    }
}

impl Config {
    fn from(opt_matches: &getopts::Matches) -> Result<Self, ()> {
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

        // let (input_file, is_input_stdin) = if !opt_matches.free.is_empty() {
        //     if opt_matches.free[0] == "-" {
        //         (InputFile::StdIn, true)
        //     } else {
        //         (InputFile::Path(PathBuf::from(&opt_matches.free[0])), false)
        //     }
        // } else {
        //     return Err(());
        // };

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
            // inline: match (
            //     opt_matches.opt_present("inline"),
            //     opt_matches.opt_present("replace"),
            //     is_input_stdin,
            // ) {
            //     (true, true, false) => Inline::InlineAndReplace,
            //     (true, false, _) => Inline::Inline,
            //     (false, false, _) => Inline::None,
            //     _ => return Err(()),
            // },
        })
    }
}

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
        .optflag("", "no-header", "Exclude the header of ToC");
    // .optflag(
    //     "i",
    //     "inline",
    //     "With this flag, the full markdown file will be printed with ToC."
    // )
    // .optflag(
    //     "",
    //     "replace", "Should be used with --inline option and FILE should not be stdin. The original file will be replace instead of printing to standard output."
    // );

    match opts.parse(args) {
        Ok(m) => {
            if m.opt_present("h") {
                Err(())
            } else {
                Config::from(&m)
            }
        }
        Err(_) => Err(()),
    }
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
    }.unwrap();

    println!("");

    if let Some(ref header) = config.header {
        println!("{}\n", header);
    }

    content
        .lines()
        .map(Heading::from_str)
        .filter_map(Result::ok)
        .for_each(|h| {
            if h.depth >= config.min_depth && config.max_depth.map(|d| h.depth <= d).unwrap_or(true)
            {
                println!(
                    "{}{} {}",
                    " ".repeat(config.indent).repeat(h.depth - config.min_depth),
                    &config.bullet,
                    if config.no_link {
                        h.title
                    } else {
                        format!("[{}](#{})", h.title, slugify!(&h.title),)
                    }
                );
            }
        });
}
