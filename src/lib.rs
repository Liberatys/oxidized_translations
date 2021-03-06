//! Oxidized translations is a rust port of the I18n gem.
//!
//! The purpose of the crate is to supply an easy to use wrapper for translation files
//! that allows easy access to translations.
//!
//! # Configuration
//!
//! Configuration can either be solved via inline configuration or reading from a yaml / toml file.
//! The configuration expects the following values to be supplied:
//!   - If '-' is used for the fallback locale, the translation searcher will throw an error rather
//! - fallback_locale: [either a value or -]
//!   trying to supply a translation from the fallback language
//! - translation_folder: path to the locale folder
//!   - This library is intended to be used for loading translations from a file and thus requires a
//!   path to the folder holding the locale files.
//! - default_locale: the default locale for translations
//!   - If not other locale is supplied this locale will be used for translations
//!
//! # File formats
//!
//! This crate is fairly flexible and limit at the same time!
//! It allows for files to have multiple ways of declaring a locale which are as follows
//!
//! 1. Locale identifier in the form of a leading definition e.g
//! ```yaml
//! de: [<- on first line of the file]
//! ```
//! 2. Locale within the file path. Expected at the second to last position in the file path e.g
//! ```yml
//! configs/locales/file_path.de.yml
//!
//! configs/locales/file_path.en.yml
//! ```
//!
//! The different types of locale definitions are tried by the file reader in the given order of
//! possible ways of defining the locale by file.

// TODO:
//
// Look into the ruby I18n implementation
//    - liberatys, Sun Feb 28 16:43:30 2021

#![deny(
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]

pub mod config;
mod errors;
pub mod oxidized_map;
mod reader;
