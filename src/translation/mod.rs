pub mod dictionary;
pub mod language;

pub use dictionary::{Dictionary, DictionarySystem};
pub use language::{Language, LanguageSystem};

use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, AddAssign};
use wgpu::naga::FastHashMap;

/// A single translated string: a text ID, a language ID, and the translated
/// content.
#[derive(Debug, Clone)]
pub struct Translation {
    text_id: u32,
    language_id: u32,
    text: String,
}

impl Translation {
    /// Creates a new [`Translation`] entry.
    ///
    /// * `text_id` — stable key ID (derived from the source key via
    ///   [`generate_id_from_name`]).
    /// * `language_id` — language ID (derived from the locale code via
    ///   [`generate_id_from_name`]).
    /// * `text` — the translated string for this locale.
    #[must_use]
    pub const fn new(text_id: u32, language_id: u32, text: String) -> Self {
        Self {
            text_id,
            language_id,
            text,
        }
    }

    pub(crate) fn get_translation(&self) -> &str {
        &self.text
    }
}

/// Stores translations indexed by (`text_id`, `language_id`) for O(1) lookup.
pub struct TranslationSystem {
    translations: FastHashMap<(u32, u32), Translation>,
}

impl TranslationSystem {
    /// Creates a new, empty [`TranslationSystem`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            translations: FastHashMap::default(),
        }
    }

    /// Inserts a [`Translation`] entry by name keys.
    /// The IDs are derived automatically via [`generate_id_from_name`].
    /// Overwrites any existing entry for the same key pair.
    pub fn add_translation_by_name(&mut self, text_key: &str, language_key: &str, text: String) {
        let text_id = generate_id_from_name(text_key);
        let language_id = generate_id_from_name(language_key);
        self.translations.insert(
            (text_id, language_id),
            Translation::new(text_id, language_id, text),
        );
    }

    /// Inserts a pre-built [`Translation`] entry, keyed by `(text_id, language_id)`.
    /// Overwrites any existing entry for the same key pair.
    pub fn add_translation(&mut self, translation: Translation) {
        self.translations.insert((translation.text_id, translation.language_id), translation);
    }

    /// Returns an iterator over all stored [`Translation`] entries.
    pub fn get_translations(&self) -> impl Iterator<Item = &Translation> {
        self.translations.values()
    }

    /// Looks up the translation by name keys.
    /// Returns `None` when no entry exists for the pair.
    #[must_use]
    pub fn get_translation_by_name(&self, text_key: &str, language_key: &str) -> Option<&Translation> {
        let text_id = generate_id_from_name(text_key);
        let language_id = generate_id_from_name(language_key);
        self.translations.get(&(text_id, language_id))
    }

    /// Looks up the translation for `(text_id, language_id)`.
    /// Returns `None` when no entry exists for the pair.
    #[must_use]
    pub fn get_translation(&self, text_id: u32, language_id: u32) -> Option<&Translation> {
        self.translations.get(&(text_id, language_id))
    }
}


impl Default for TranslationSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for TranslationSystem {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.translations.extend(rhs.translations);
        self
    }
}

impl AddAssign for TranslationSystem {
    fn add_assign(&mut self, rhs: Self) {
        self.translations.extend(rhs.translations);
    }
}

/// Generates an u32 ID from the language name using a hash function.
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn generate_id_from_name(name: &str) -> u32 {
    let mut s = DefaultHasher::new();
    name.hash(&mut s);
    // Cast to u32 directly truncates the upper 32 bits,
    // which is perfectly fine and standard for hashing.
    s.finish() as u32
}