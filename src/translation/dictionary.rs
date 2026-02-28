use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};

pub struct Dictionary {
    id: u32,
    text: String,
}

impl Dictionary {
    pub fn generate_id_from_name(name: &str) -> u32 {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        s.finish() as u32
    }

    pub fn new(text: &str) -> Dictionary {
        Self {
            id: Self::generate_id_from_name(text),
            text: text.to_string(),
        }
    }
}

impl PartialEq for Dictionary {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct DictionarySystem {
    dictionaries: Vec<Dictionary>,
}

impl DictionarySystem {
    pub fn new() -> Self {
        Self {
            dictionaries: Vec::new(),
        }
    }

    pub fn add_dictionary(&mut self, text: &str) {
        self.dictionaries.push(Dictionary::new(text));
    }

    pub fn get_dictionaries(&self) -> &Vec<Dictionary> {
        &self.dictionaries
    }

    pub fn get_dictionary(&self, text_id: u32) -> Option<&Dictionary> {
        self.dictionaries
            .iter()
            .find(|d| d.id == text_id)
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