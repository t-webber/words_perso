//! Downloads the definitions of the found words

use std::io::{Write as _, stdout};

use reqwest::blocking::{Response, get};

use crate::read_write::read_write;
use crate::word_generator::HrefWord;

/// Defined word
pub struct DefinedWord {
    /// Path in the file system
    pub path: String,
    /// The word
    pub word: String,
}

impl DefinedWord {
    /// Where to find the word definitions
    pub const PATH_PREFIX: &str = "data/defs/";

    /// Returns an appropriate path to store the word's definition
    fn from_word(word: String) -> Self {
        Self {
            path: format!("{}{}.html", Self::PATH_PREFIX, word.replace('/', "-slash-")),
            word,
        }
    }
}

/// Stores the status of the definition
enum WordDefinition {
    /// The word was downloaded
    Found(DefinedWord),
    /// The word doesn't have a definition
    Invalid,
    /// The word wasn't downloaded yet
    NotDownloaded,
}

/// Download a given page
fn download_page(url: &str) -> Result<String, String> {
    print!(" {url}");
    stdout()
        .flush()
        .map_err(|err| format!("Failed to print to stdout: {err}."))?;
    get(url)
        .and_then(Response::text)
        .map_err(|err| format!("Failed to fetch url {url}:\n{err}"))
}

/// Get the definitions of one word
fn get_definition(href_word: HrefWord) -> Result<WordDefinition, String> {
    let def = if let Some(url) = href_word.to_url() {
        let defined_word = DefinedWord::from_word(href_word.word);
        if cfg!(feature = "download") {
            let _def = read_write(&defined_word.path, || download_page(&url))?;
            WordDefinition::Found(defined_word)
        } else {
            WordDefinition::NotDownloaded
        }
    } else {
        WordDefinition::Invalid
    };
    Ok(def)
}

/// Get the definitions of the given words
#[expect(
    clippy::arithmetic_side_effects,
    reason = "input is deterministic & bounds never reached & sub checked"
)]
#[expect(
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    clippy::as_conversions,
    reason = "approximation wanted"
)]
pub fn get_definitions(words: Box<[HrefWord]>) -> Box<[DefinedWord]> {
    let slice: usize = 10_000;
    let mut count: usize = 0;
    let mut invalid = 0.;
    words
        .into_iter()
        .filter_map(|href_word| {
            print!("({})", &href_word.word);
            let defined_word = match get_definition(href_word) {
                Ok(WordDefinition::Found(defined_word)) => {
                    count += 1;
                    Some(defined_word)
                }
                Ok(WordDefinition::NotDownloaded) => {
                    count += 1;
                    None
                }
                Ok(WordDefinition::Invalid) => {
                    count += 1;
                    invalid += 1.;
                    None
                }
                Err(_err) => None,
            };
            print!("\r{:100}\r", "");
            if (count + 1) % slice == 0 {
                println!(
                    "[{:6}-{:6}] {:3}%",
                    (count + 1) - slice,
                    count,
                    100. * invalid / (slice as f32)
                );
                invalid = 0.;
            }
            defined_word
        })
        .collect()
}
