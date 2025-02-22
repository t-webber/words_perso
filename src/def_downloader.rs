//! Downloads the definitions of the found words

use std::fs::{read_to_string, write};
use std::io::{Write as _, stdout};

use reqwest::blocking::{Response, get};

use crate::word_generator::HrefWord;

/// Stores the status of the definition
enum WordDefinition {
    /// The word was downloaded
    Found(String),
    /// The word doesn't have a definition
    Invalid,
    /// The word wasn't downloaded yet
    NotDownloaded,
}

/// Download a given page
fn download_page(url: &str) -> Result<String, ()> {
    print!(" {url}");
    stdout().flush().map_err(|_err| ())?;
    get(url).and_then(Response::text).map_err(|_err| ())
}

/// Get the definitions of one word
fn get_definition(href_word: &HrefWord) -> Result<WordDefinition, ()> {
    let def = if let Some(url) = href_word.to_url() {
        let path = href_word.to_path();
        match read_to_string(&path) {
            Ok(def) => WordDefinition::Found(def),
            Err(_) => {
                if cfg!(feature = "download") {
                    let def = download_page(&url)?;
                    write(path, &def).map_err(|_err| ())?;
                    WordDefinition::Found(def)
                } else {
                    WordDefinition::NotDownloaded
                }
            }
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
pub fn get_definitions(words: &[HrefWord]) {
    let slice: usize = 10_000;
    let mut count: usize = 0;
    let mut invalid = 0.;
    for href_word in words {
        print!("({})", &href_word.word);
        match get_definition(href_word) {
            Ok(WordDefinition::Found(_) | WordDefinition::NotDownloaded) => count += 1,
            Ok(WordDefinition::Invalid) => {
                count += 1;
                invalid += 1.;
            }
            Err(()) => (),
        }
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
    }
}
