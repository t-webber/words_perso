#![feature(let_chains)]

use std::fs::read_to_string;

use html_parser::prelude::{Html, parse_html};

const LIST_PATHS: [&str; 16] = [
    "data/lists/001-010.html",
    "data/lists/011-020.html",
    "data/lists/021-030.html",
    "data/lists/031-040.html",
    "data/lists/041-050.html",
    "data/lists/051-060.html",
    "data/lists/061-070.html",
    "data/lists/071-080.html",
    "data/lists/081-090.html",
    "data/lists/091-100.html",
    "data/lists/101-110.html",
    "data/lists/111-120.html",
    "data/lists/121-130.html",
    "data/lists/131-140.html",
    "data/lists/141-150.html",
    "data/lists/151-157.html",
];

struct Word {
    href: String,
    word: String,
}

fn parse_list(list_path: &str) -> Vec<Word> {
    let list = read_to_string(list_path)
        .unwrap_or_else(|err| panic!("No such file or directory: {list_path}.\n{err}"));

    let html = parse_html(&list).unwrap();
    if let Html::Vec(vec) = html {
        vec.into_iter()
            .filter_map(|link| {
                if let Html::Tag { tag, child, .. } = link
                    && let Some(href) = tag.into_attr_value("href")
                    && let Html::Text(word) = *child
                {
                    Some(Word { word, href })
                } else {
                    None
                }
            })
            .collect()
    } else {
        unreachable!("Invalid input")
    }
}

fn parse_lists(list_paths: &[&str]) -> Vec<Word> {
    let mut words = Vec::with_capacity(list_paths.len() * 10_000);
    for list_path in list_paths {
        words.extend(parse_list(list_path));
    }
    words
}

fn main() {
    let words = parse_lists(&LIST_PATHS);
    dbg!(words.len());
}
