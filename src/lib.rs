#[macro_use]
extern crate slugify;

use slugify::slugify;
use std::path::PathBuf;
use std::str::FromStr;

pub struct Heading {
    pub depth: usize,
    pub title: String,
}

impl FromStr for Heading {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim_right();
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

impl Heading {
    pub fn format(&self, config: &Config) -> Option<String> {
        if self.depth >= config.min_depth
            && config.max_depth.map(|d| self.depth <= d).unwrap_or(true)
        {
            Some(format!(
                "{}{} {}",
                " ".repeat(config.indent)
                    .repeat(self.depth - config.min_depth),
                &config.bullet,
                if config.no_link {
                    self.title.clone()
                } else {
                    format!("[{}](#{})", &self.title, slugify!(&self.title))
                }
            ))
        } else {
            None
        }
    }
}

pub enum InputFile {
    Path(PathBuf),
    StdIn,
}

// enum Inline {
//     None,
//     Inline,
//     InlineAndReplace,
// }

pub struct Config {
    pub input_file: InputFile,
    pub bullet: String,
    pub indent: usize,
    pub max_depth: Option<usize>,
    pub min_depth: usize,
    pub header: Option<String>,
    pub no_link: bool,
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
