pub mod dictionary;
pub mod language;

pub use dictionary::{Dictionary, DictionarySystem};
pub use language::{Language, LanguageSystem};

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, AddAssign};

pub struct Translation {
    text_id: u32,
    language_id: u32,
    text: String,
}

impl Translation {
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
    translations: HashMap<(u32, u32), Translation>,
}

impl TranslationSystem {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
        }
    }

    pub fn add_translation(&mut self, translation: Translation) {
        self.translations.insert((translation.text_id, translation.language_id), translation);
    }

    pub fn get_translations(&self) -> impl Iterator<Item = &Translation> {
        self.translations.values()
    }

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