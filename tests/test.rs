extern crate markdown_toc;

use markdown_toc::*;
use std::str::FromStr;

static TEST_DOC: &'static str = include_str!("test.md");
static OUTPUT: &'static str = include_str!("output");

#[test]
fn test() {
    let content = String::from(TEST_DOC);

    let heads: Vec<_> = content
        .lines()
        .map(Heading::from_str)
        .filter_map(Result::ok)
        .collect();

    let expected_vec: Vec<_> = OUTPUT.lines().collect();

    let first_config = Config {
        bullet: "-".to_string(),
        indent: 2,
        ..Default::default()
    };
    let second_config = Config {
        max_depth: Some(1),
        min_depth: 1,
        no_link: true,
        ..Default::default()
    };

    let actual_vec: Vec<_> = heads
        .iter()
        .map(|h| format!("{}, {}", &h.title, &h.depth))
        .chain(heads.iter().filter_map(|h| h.format(&first_config)))
        .chain(heads.iter().filter_map(|h| h.format(&second_config)))
        .collect();

    assert_eq!(actual_vec.len(), expected_vec.len());
    for (actual, expected) in actual_vec.into_iter().zip(expected_vec) {
        assert_eq!(actual, expected);
    }
}
