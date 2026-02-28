use std::ops::{Add, AddAssign};

pub struct Translation {
    text_id: u32,
    language_id: u32,
    translation: String,
}

impl Translation {
    pub fn new(text_id: u32, language_id: u32, translation: String) -> Self {
        Self {
            text_id,
            language_id,
            translation,
        }
    }

    pub(crate) fn get_translation(&self) -> &str {
        &self.translation
    }
    pub(crate) fn get_language(&self) -> &u32 {
        &self.language_id
    }
    pub(crate) fn get_text(&self) -> &u32 {
        &self.text_id
    }
}

pub struct TranslationSystem {
    translations: Vec<Translation>,
}
impl TranslationSystem {
    pub fn new() -> Self {
        Self {
            translations: Vec::new(),
        }
    }

    pub fn add_translation(&mut self, translation: Translation) {
        self.translations.push(translation);
    }

    pub fn get_translations(&self) -> &Vec<Translation> {
        &self.translations
    }

    pub fn get_translation(
        &self,
        text_id: u32,
    ) -> Option<&Translation> {
        self.translations
            .iter()
            .find(|t| t.text_id == text_id)
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