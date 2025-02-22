//! Extracts the english part of the definitions

use std::fs::read_to_string;

use html_parser::prelude::{Filter, parse_html};

use crate::def_downloader::DefinedWord;
use crate::read_write::read_write;

/// Word with only the english definition
pub struct EnglishWord {
    /// Path to the english definition
    pub path: String,
    /// The word
    pub word: String,
}

impl EnglishWord {
    /// Where to find the word english definitions
    pub const PATH_PREFIX: &str = "data/en/";

    /// Convert a path from a [`DefinedWord`] to an [`EnglishWord`].
    fn convert_path(path: &str) -> String {
        path.replace(DefinedWord::PATH_PREFIX, Self::PATH_PREFIX)
    }
}

impl From<DefinedWord> for EnglishWord {
    fn from(DefinedWord { path, word }: DefinedWord) -> Self {
        Self {
            word,
            path: path.replace(DefinedWord::PATH_PREFIX, Self::PATH_PREFIX),
        }
    }
}

/// Extracts the english part of the definitions
pub fn extract_english(words: Box<[DefinedWord]>) -> Box<[EnglishWord]> {
    words
        .into_iter()
        .map(|defined_word| {
            let word = defined_word.word;
            let path = EnglishWord::convert_path(&defined_word.path);
            let _def = extract_english_word(&defined_word.path, &path, &word);
            EnglishWord { path, word }
        })
        .collect()
}

/// Extracts the english part a word's definition
#[expect(clippy::expect_used, reason = "unreachable with message")]
#[expect(clippy::panic, reason = "fail to check my html crate works")]
pub fn extract_english_word(input_path: &str, output_path: &str, word: &str) -> String {
    read_write(output_path, || {
        let definition = read_to_string(input_path).expect("All defined words are written");
        let html = parse_html(&definition)
            .unwrap_or_else(|err| panic!("Invalid HTML on word {word}:\n{err}"));
        let filter = Filter::new().attribute_value("id", "English").depth(2);
        let english_section = html.filter(&filter);
        Ok(english_section.to_string())
    })
    .expect("Always returns Ok")
}
