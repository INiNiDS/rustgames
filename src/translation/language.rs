use crate::translation::generate_id_from_name;
use aam_rs::aaml::AAML;
use std::ops::{Add, AddAssign};
use wgpu::naga::FastHashMap;

/// Represents a language with a unique identifier, a short locale code, and a full display name.
#[derive(Debug, Clone)]
pub struct Language {
    /// Unique numeric identifier derived from the language's short name.
    pub id: u32,
    /// Short locale code for the language (e.g. `"en_us"`).
    pub small_name: String,
    /// Full human-readable name of the language (e.g. `"English"`).
    pub full_name: String,
}

impl Language {
    /// Resolves a [`Language`] by looking up `name` in the translation file.
    ///
    /// If `name` contains an underscore it is treated as a locale code
    /// (`small_name`) and the file is expected to map it to the full name.
    /// Otherwise, `name` is treated as the full name and the file is expected
    /// to map it to the locale code.
    ///
    /// Returns `None` if the translation file cannot be loaded or the entry
    /// is not found.
    #[must_use]
    pub fn resolve(name: &str) -> Option<Self> {
        let translations = AAML::load("src/static/translation.aam").ok()?;

        if name.contains('_') {
            let full = translations.find_obj(name)?;
            Some(Self {
                id: generate_id_from_name(name),
                small_name: name.to_string(),
                full_name: full.to_string(),
            })
        } else {
            let small = translations.find_obj(name)?;
            Some(Self {
                id: generate_id_from_name(&small),
                small_name: small.to_string(),
                full_name: name.to_string(),
            })
        }
    }

    /// Creates a new [`Language`] directly from its locale code and full name.
    ///
    /// The numeric `id` is derived automatically from `small_name`.
    #[must_use]
    pub fn new(small_name: String, full_name: String) -> Self {
        Self {
            id: generate_id_from_name(&small_name),
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

/// Manages a collection of [`Language`] entries and tracks the currently active language.
pub struct LanguageSystem {
    /// Keyed by language id for O(1) lookup
    languages: FastHashMap<u32, Language>,
    /// Currently active language id (0 = none set)
    current_language_id: u32,
}

impl LanguageSystem {
    /// Creates a new, empty [`LanguageSystem`] with no active language.
    #[must_use]
    pub fn new() -> Self {
        Self {
            languages: FastHashMap::default(),
            current_language_id: 0,
        }
    }

    /// Inserts `language` into the system, keyed by its [`Language::id`].
    ///
    /// If a language with the same id already exists it will be replaced.
    pub fn add_language(&mut self, language: Language) {
        self.languages.insert(language.id, language);
    }

    /// Sets the currently active language by its short locale code (e.g. `"en_us"`).
    ///
    /// Does nothing if no language with that name is registered.
    /// Use this instead of [`set_current_language`] to avoid dealing with raw IDs.
    pub fn set_current_language_by_name(&mut self, small_name: &str) {
        if let Some(lang) = self.get_language_by_small_name(small_name) {
            self.current_language_id = lang.id;
        }
    }

    /// Sets the currently active language by its numeric `id`.
    ///
    /// Prefer [`set_current_language_by_name`] unless you already hold the id.
    /// Pass `0` to clear the active language.
    pub const fn set_current_language(&mut self, id: u32) {
        self.current_language_id = id;
    }

    /// Returns a reference to the currently active [`Language`], or `None`
    /// if no language has been set.
    #[must_use]
    pub fn get_current_language(&self) -> Option<&Language> {
        if self.current_language_id == 0 {
            None
        } else {
            self.languages.get(&self.current_language_id)
        }
    }

    /// Returns a reference to the [`Language`] with the given numeric `id`,
    /// or `None` if it is not registered.
    #[must_use]
    pub fn get_language_by_id(&self, id: u32) -> Option<&Language> {
        self.languages.get(&id)
    }

    /// Returns a reference to the [`Language`] whose [`Language::small_name`]
    /// matches `small_name`, or `None` if not found.
    #[must_use]
    pub fn get_language_by_small_name(&self, small_name: &str) -> Option<&Language> {
        self.languages
            .values()
            .find(|lang| lang.small_name == small_name)
    }

    /// Returns a reference to the [`Language`] whose [`Language::full_name`]
    /// matches `full_name`, or `None` if not found.
    #[must_use]
    pub fn get_language_by_full_name(&self, full_name: &str) -> Option<&Language> {
        self.languages
            .values()
            .find(|lang| lang.full_name == full_name)
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
