use aam_rs::aaml::AAML;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone)]
pub struct Language {
    pub id: u32,
    pub small_name: String,
    pub full_name: String,
}


impl Language {
    pub fn generate_id_from_name(name: &str) -> u32 {
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        s.finish() as u32
    }

    pub fn resolve(name: &str) -> Option<Self> {
        let translations = AAML::load("src/static/translation.aam").ok()?;

        if name.contains('_') {
            let full = translations.find_obj(name)?;
            Some(Self {
                id: Self::generate_id_from_name(name),
                small_name: name.to_string(),
                full_name: full.to_string(),
            })
        } else {
            let small = translations.find_obj(name)?;
            Some(Self {
                id: Self::generate_id_from_name(&small),
                small_name: small.to_string(),
                full_name: name.to_string(),
            })
        }
    }

    pub fn new(small_name: String, full_name: String) -> Self {
        Self {
            id: Self::generate_id_from_name(&small_name),
            small_name,
            full_name,
        }
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct LanguageSystem {
    /// Keyed by language id for O(1) lookup
    languages: HashMap<u32, Language>,
    /// Currently active language id (0 = none set)
    current_language_id: u32,
}

impl LanguageSystem {
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
            current_language_id: 0,
        }
    }

    pub fn add_language(&mut self, language: Language) {
        self.languages.insert(language.id, language);
    }

    pub fn set_current_language(&mut self, id: u32) {
        self.current_language_id = id;
    }

    pub fn set_current_language_by_name(&mut self, small_name: &str) {
        if let Some(lang) = self.get_language_by_small_name(small_name) {
            self.current_language_id = lang.id;
        }
    }

    pub fn get_current_language(&self) -> Option<&Language> {
        if self.current_language_id == 0 {
            None
        } else {
            self.languages.get(&self.current_language_id)
        }
    }

    pub fn get_language_by_id(&self, id: u32) -> Option<&Language> {
        self.languages.get(&id)
    }

    pub fn get_language_by_small_name(&self, small_name: &str) -> Option<&Language> {
        self.languages.values().find(|lang| lang.small_name == small_name)
    }

    pub fn get_language_by_full_name(&self, full_name: &str) -> Option<&Language> {
        self.languages.values().find(|lang| lang.full_name == full_name)
    }
}

impl Default for LanguageSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for LanguageSystem {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.languages.extend(rhs.languages);
        self
    }
}

impl AddAssign for LanguageSystem {
    fn add_assign(&mut self, rhs: Self) {
        self.languages.extend(rhs.languages);
    }
}