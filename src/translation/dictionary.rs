use crate::translation::generate_id_from_name;
use std::ops::{Add, AddAssign};
use wgpu::naga::FastHashMap;

/// A single fallback text entry keyed by a stable numeric ID.
#[derive(Debug, Clone)]
pub struct Dictionary {
    id: u32,
    text: String,
}

impl Dictionary {
    /// Creates a new [`Dictionary`] entry. The numeric `id` is derived from
    /// `text` via [`generate_id_from_name`].
    #[must_use]
    pub fn new(text: &str) -> Self {
        Self {
            id: generate_id_from_name(text),
            text: text.to_string(),
        }
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
    dictionaries: FastHashMap<u32, Dictionary>,
}

impl DictionarySystem {
    /// Creates a new, empty [`DictionarySystem`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            dictionaries: FastHashMap::default(),
        }
    }

    /// Adds a fallback entry whose ID is derived from `text` via
    /// [`generate_id_from_name`].
    pub fn add_dictionary(&mut self, text: &str) {
        let d = Dictionary::new(text);
        self.dictionaries.insert(d.id, d);
    }

    /// Adds a fallback entry with an explicit numeric `id`.
    pub fn add_dictionary_entry(&mut self, id: u32, text: &str) {
        self.dictionaries.insert(id, Dictionary { id, text: text.to_string() });
    }

    /// Returns an iterator over all stored [`Dictionary`] entries.
    pub fn get_dictionaries(&self) -> impl Iterator<Item = &Dictionary> {
        self.dictionaries.values()
    }

    /// Looks up the fallback entry for `text_id`.
    /// Returns `None` when no entry exists.
    #[must_use]
    pub fn get_dictionary(&self, text_id: u32) -> Option<&Dictionary> {
        self.dictionaries.get(&text_id)
    }

    /// Looks up the fallback entry by name.
    /// The ID is derived automatically via [`generate_id_from_name`].
    /// Returns `None` when no entry exists.
    #[must_use]
    pub fn get_dictionary_by_name(&self, text: &str) -> Option<&Dictionary> {
        let id = generate_id_from_name(text);
        self.dictionaries.get(&id)
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