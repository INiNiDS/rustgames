use std::collections::HashMap;
use std::ops::{Add, AddAssign};

pub struct Translation {
    text_id: u32,
    language_id: u32,
    translation: String,
}

impl Translation {
    pub const fn new(text_id: u32, language_id: u32, translation: String) -> Self {
        Self {
            text_id,
            language_id,
            translation,
        }
    }

    pub(crate) fn get_translation(&self) -> &str {
        &self.translation
    }
}

/// Stores translations indexed by (`text_id`, `language_id`) for O(1) lookup.
pub struct TranslationSystem {
    translations: HashMap<(u32, u32), Translation>,
}

impl TranslationSystem {
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