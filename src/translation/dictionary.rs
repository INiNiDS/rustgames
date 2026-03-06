use crate::error::TextError;
use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use crate::translation::generate_id_from_name;

pub struct Dictionary {
    id: u32,
    text: String,
}

impl Dictionary {

    /// # Errors
    /// Returns [`TextError::HashIdOverflow`] if the hash of `text` exceeds `u32::MAX`.
    pub fn new(text: &str) -> Result<Self, TextError> {
        Ok(Self {
            id: generate_id_from_name(text),
            text: text.to_string(),
        })
    }

    pub(crate) fn get_text(&self) -> &str {
        &self.text
    }
}

impl PartialEq for Dictionary {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Stores fallback display texts indexed by their id for O(1) lookup.
pub struct DictionarySystem {
    dictionaries: HashMap<u32, Dictionary>,
}

impl DictionarySystem {
    #[must_use] 
    pub fn new() -> Self {
        Self {
            dictionaries: HashMap::new(),
        }
    }

    /// # Errors
    /// Returns [`TextError::HashIdOverflow`] if the hash of `text` exceeds `u32::MAX`.
    pub fn add_dictionary(&mut self, text: &str) -> Result<(), TextError> {
        let d = Dictionary::new(text)?;
        self.dictionaries.insert(d.id, d);
        Ok(())
    }

    pub fn add_dictionary_entry(&mut self, id: u32, text: &str) {
        self.dictionaries.insert(id, Dictionary { id, text: text.to_string() });
    }

    pub fn get_dictionaries(&self) -> impl Iterator<Item = &Dictionary> {
        self.dictionaries.values()
    }

    #[must_use] 
    pub fn get_dictionary(&self, text_id: u32) -> Option<&Dictionary> {
        self.dictionaries.get(&text_id)
    }
}

impl Default for DictionarySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for DictionarySystem {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.dictionaries.extend(rhs.dictionaries);
        self
    }
}

impl AddAssign for DictionarySystem {
    fn add_assign(&mut self, rhs: Self) {
        self.dictionaries.extend(rhs.dictionaries);
    }
}