//! Generates the list of words

#![expect(clippy::panic, reason = "the code execution is deterministic")]

use std::fs::read_to_string;

use html_parser::prelude::{Html, parse_html};

/// Struct to represent a word
pub struct HrefWord {
    /// Link to the wiktionary definition
    pub href: String,
    /// The word
    pub word: String,
}

impl HrefWord {
    /// Check if a word has valid link, i.e. that the definition exists
    ///
    /// The [`HrefWord`] is considered valid iff the link is valid, i.e., iff
    /// the definition exists on the wiktionary.
    pub fn is_valid(&self) -> bool {
        self.href.starts_with("/wiki/")
    }

    /// Returns the full URL to the definition of the word
    pub fn to_url(&self) -> Option<String> {
        self.is_valid()
            .then(|| format!("https://en.wiktionary.org/{}", self.href))
    }
}

/// Parses a list of HTML files that contain lists of words
pub fn parse_lists(list_paths: &[&str]) -> Box<[HrefWord]> {
    list_paths
        .iter()
        .flat_map(|list_path| {
            let list = read_to_string(list_path)
                .unwrap_or_else(|err| panic!("No such file or directory: {list_path}.\n{err}"));
            let html = parse_html(&list).unwrap_or_else(|err| panic!("Invalid input.\n{err}"));
            if let Html::Vec(vec) = html {
                vec.into_iter().filter_map(|link| {
                    if let Html::Tag { tag, child, .. } = link
                        && let Some(href) = tag.into_attr_value("href")
                        && let Html::Text(word) = *child
                    {
                        Some(HrefWord { href, word })
                    } else {
                        None
                    }
                })
            } else {
                panic!("Invalid input")
            }
        })
        .collect()
}
