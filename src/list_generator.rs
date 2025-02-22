//! Generates a list of words under txt format for words that fullfil a specific
//! condition.

#![expect(clippy::panic, reason = "the code execution is deterministic")]

use std::fs::OpenOptions;
use std::io::Write as _;

use crate::word_generator::HrefWord;

/// Macro to handle all the calls o [`generate_list`] in one go
macro_rules! generate_list_generators {
    ($words:ident; $($condition:expr, $path:expr;)*) => {
        $(
            generate_list($words, $condition, $path);
        )*
    };
}

/// Generate the list of words that follow the given rule
fn generate_list<F: Fn(&HrefWord) -> bool>(words: &[HrefWord], condition: F, file: &str) {
    let path = format!("data/txt/{file}.txt");
    let mut fd = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&path)
        .unwrap_or_else(|err| {
            panic!("Permission denied. Failed to access location {path}.\n{err}")
        });
    words
        .iter()
        .filter(|href_word| condition(href_word))
        .try_for_each(|href_word| writeln!(fd, "{}", href_word.word))
        .unwrap_or_else(|err| panic!("Permission denied. Failed to write to file {path}.\n{err}",));
}

/// Generate lists of words for a given set of conditions
pub fn generate_lists(words: &[HrefWord]) {
    generate_list_generators!(words;
    |_| true, "all";
    |href_word| is_alpha_lower(&href_word.word), "alpha_lower";
    |href_word| href_word.word.len() >= 3, "min3";
    HrefWord::is_valid, "valid";
    |href_word| href_word.word.len() >= 3 && is_alpha_lower(&href_word.word), "alpha_lower_min3";
    |href_word| is_alpha_lower(&href_word.word) && href_word.is_valid(), "alpha_lower_valid";
    |href_word| href_word.word.len() >= 3 && href_word.is_valid(), "min3_valid";
    |href_word| href_word.word.len() >= 3 && is_alpha_lower(&href_word.word) && href_word.is_valid(), "alpha_lower_min3_valid";
    );
}

/// Checks if a string is composed of only alphabetic ascii lower characters.
fn is_alpha_lower(word: &str) -> bool {
    word.chars().all(|ch| ch.is_ascii_lowercase())
}
