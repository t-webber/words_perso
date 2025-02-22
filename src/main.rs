//! Scrapes the wiktionary for the definition of the 150000 most used english
//! words.

#![warn(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![expect(clippy::multiple_crate_versions, reason = "dependencies need this")]
#![expect(
    clippy::blanket_clippy_restriction_lints,
    reason = "I want all the lints"
)]
#![expect(
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::question_mark_used,
    clippy::integer_division_remainder_used,
    reason = "bad lint"
)]
#![expect(clippy::print_stdout, reason = "logging")]
#![feature(let_chains)]

mod def_downloader;
mod list_generator;
mod word_generator;

/// Paths to the lists of words
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

fn main() {
    println!("Extracting words...");
    let words = word_generator::parse_lists(&LIST_PATHS);
    println!("Found {} words", words.len());

    #[cfg(feature = "lists")]
    println!("Generating lists...");
    list_generator::generate_lists(&words);

    println!("Fetching definitions lists...");
    def_downloader::get_definitions(&words);
}
