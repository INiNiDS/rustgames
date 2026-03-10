//! Tests for the translation subsystem:
//! [`TranslationSystem`], [`LanguageSystem`], [`DictionarySystem`],
//! and [`generate_id_from_name`].

use rustgames::translation::{
    generate_id_from_name, Dictionary, DictionarySystem, Language, LanguageSystem,
    TranslationSystem,
};

// ─────────────────────────────────────────────────────────────────────────────
// generate_id_from_name
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn generate_id_same_string_returns_same_id() {
    let id1 = generate_id_from_name("en_us");
    let id2 = generate_id_from_name("en_us");
    assert_eq!(id1, id2, "same input must produce same ID");
}

#[test]
fn generate_id_different_strings_different_ids() {
    let id_en = generate_id_from_name("en_us");
    let id_ru = generate_id_from_name("ru_ru");
    assert_ne!(id_en, id_ru, "different locales should produce different IDs");
}

#[test]
fn generate_id_empty_string_is_deterministic() {
    let id1 = generate_id_from_name("");
    let id2 = generate_id_from_name("");
    assert_eq!(id1, id2);
}

#[test]
fn generate_id_case_sensitive() {
    let lower = generate_id_from_name("hello");
    let upper = generate_id_from_name("HELLO");
    assert_ne!(lower, upper, "IDs are case-sensitive");
}

#[test]
fn generate_id_stress_uniqueness() {
    let ids: Vec<u32> = (0u32..200)
        .map(|i| generate_id_from_name(&format!("locale_{i:03}")))
        .collect();
    let unique: std::collections::HashSet<u32> = ids.iter().copied().collect();
    assert_eq!(unique.len(), ids.len(), "all 200 IDs should be unique");
}

// ─────────────────────────────────────────────────────────────────────────────
// TranslationSystem
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn translation_system_starts_empty() {
    let sys = TranslationSystem::new();
    assert_eq!(sys.get_translations().count(), 0);
}

#[test]
fn translation_system_add_and_lookup() {
    let mut sys = TranslationSystem::new();
    sys.add_translation_by_name("greeting", "en_us", "Hello".to_string());
    assert!(sys.get_translation_by_name("greeting", "en_us").is_some(), "should find entry");
}

#[test]
fn translation_system_lookup_missing_returns_none() {
    let sys = TranslationSystem::new();
    assert!(sys.get_translation_by_name("nonexistent", "xx_xx").is_none());
}

#[test]
fn translation_system_overwrite_entry() {
    let mut sys = TranslationSystem::new();
    sys.add_translation_by_name("key", "en_us", "First".to_string());
    // Overwrite with new text — count must stay 1
    sys.add_translation_by_name("key", "en_us", "Second".to_string());
    assert_eq!(sys.get_translations().count(), 1, "overwrite should not add a duplicate entry");
    assert!(sys.get_translation_by_name("key", "en_us").is_some());
}

#[test]
fn translation_system_multiple_languages() {
    let mut sys = TranslationSystem::new();
    sys.add_translation_by_name("greet", "en_us", "Hello".to_string());
    sys.add_translation_by_name("greet", "ru_ru", "Привет".to_string());
    assert!(sys.get_translation_by_name("greet", "en_us").is_some());
    assert!(sys.get_translation_by_name("greet", "ru_ru").is_some());
    assert_eq!(sys.get_translations().count(), 2);
}

#[test]
fn translation_system_add_operator_merges() {
    let mut a = TranslationSystem::new();
    let mut b = TranslationSystem::new();
    a.add_translation_by_name("k", "aa", "AA".to_string());
    b.add_translation_by_name("k", "bb", "BB".to_string());
    let merged = a + b;
    assert_eq!(merged.get_translations().count(), 2);
    assert!(merged.get_translation_by_name("k", "aa").is_some());
    assert!(merged.get_translation_by_name("k", "bb").is_some());
}

#[test]
fn translation_system_add_assign_merges() {
    let mut sys = TranslationSystem::new();
    let mut extra = TranslationSystem::new();
    sys.add_translation_by_name("k2", "en", "Yes".to_string());
    extra.add_translation_by_name("k2", "de", "Ja".to_string());
    sys += extra;
    assert_eq!(sys.get_translations().count(), 2);
}

#[test]
fn translation_system_default_equals_new() {
    let default_sys = TranslationSystem::default();
    assert_eq!(default_sys.get_translations().count(), 0);
}

#[test]
fn translation_system_stress_100_entries() {
    let mut sys = TranslationSystem::new();
    for i in 0u32..100 {
        sys.add_translation_by_name(&format!("key_{i}"), &format!("lang_{i}"), format!("text_{i}"));
    }
    assert_eq!(sys.get_translations().count(), 100);
    for i in 0u32..100 {
        assert!(
            sys.get_translation_by_name(&format!("key_{i}"), &format!("lang_{i}")).is_some(),
            "missing entry {i}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Language & LanguageSystem
// ─────────────────────────────────────────────────────────────────────────────

fn make_language(small: &str, full: &str) -> Language {
    Language::new(small.to_string(), full.to_string())
}

#[test]
fn language_new_derives_id_from_small_name() {
    let lang = make_language("en_us", "English");
    assert_eq!(lang.id, generate_id_from_name("en_us"));
    assert_eq!(lang.small_name, "en_us");
    assert_eq!(lang.full_name, "English");
}

#[test]
fn language_partial_eq_compares_by_id() {
    let a = make_language("en_us", "English");
    let b = make_language("en_us", "Different Full Name");
    assert_eq!(a, b, "same small_name → same id → equal");
}

#[test]
fn language_different_small_names_not_equal() {
    let a = make_language("en_us", "English");
    let b = make_language("ru_ru", "Russian");
    assert_ne!(a, b);
}

#[test]
fn language_system_starts_empty() {
    let sys = LanguageSystem::new();
    assert!(sys.get_current_language().is_none());
}

#[test]
fn language_system_add_and_lookup_by_id() {
    let mut sys = LanguageSystem::new();
    let lang = make_language("en_us", "English");
    let id = lang.id;
    sys.add_language(lang);
    let found = sys.get_language_by_id(id).expect("should find language");
    assert_eq!(found.small_name, "en_us");
}

#[test]
fn language_system_set_current_language() {
    let mut sys = LanguageSystem::new();
    let lang = make_language("fr_fr", "French");
    let id = lang.id;
    sys.add_language(lang);
    sys.set_current_language(id);
    let current = sys.get_current_language().expect("should have current language");
    assert_eq!(current.small_name, "fr_fr");
}

#[test]
fn language_system_set_current_language_by_name() {
    let mut sys = LanguageSystem::new();
    sys.add_language(make_language("de_de", "German"));
    sys.set_current_language_by_name("de_de");
    let current = sys.get_current_language().expect("should have current language");
    assert_eq!(current.full_name, "German");
}

#[test]
fn language_system_no_current_when_id_zero() {
    let mut sys = LanguageSystem::new();
    sys.add_language(make_language("ja_jp", "Japanese"));
    // id 0 means "no language set"
    sys.set_current_language(0);
    assert!(sys.get_current_language().is_none());
}

#[test]
fn language_system_lookup_by_small_name() {
    let mut sys = LanguageSystem::new();
    sys.add_language(make_language("zh_cn", "Chinese"));
    let found = sys.get_language_by_small_name("zh_cn").expect("should find");
    assert_eq!(found.full_name, "Chinese");
}

#[test]
fn language_system_lookup_by_full_name() {
    let mut sys = LanguageSystem::new();
    sys.add_language(make_language("es_es", "Spanish"));
    let found = sys.get_language_by_full_name("Spanish").expect("should find");
    assert_eq!(found.small_name, "es_es");
}

#[test]
fn language_system_missing_returns_none() {
    let sys = LanguageSystem::new();
    assert!(sys.get_language_by_id(u32::MAX).is_none());
    assert!(sys.get_language_by_small_name("xx_xx").is_none());
    assert!(sys.get_language_by_full_name("Klingon").is_none());
}

#[test]
fn language_system_add_operator_merges() {
    let mut a = LanguageSystem::new();
    let mut b = LanguageSystem::new();
    a.add_language(make_language("en_us", "English"));
    b.add_language(make_language("ru_ru", "Russian"));
    let merged = a + b;
    assert!(merged.get_language_by_small_name("en_us").is_some());
    assert!(merged.get_language_by_small_name("ru_ru").is_some());
}

#[test]
fn language_system_add_assign_merges() {
    let mut sys = LanguageSystem::new();
    let mut extra = LanguageSystem::new();
    sys.add_language(make_language("pt_br", "Portuguese"));
    extra.add_language(make_language("ko_kr", "Korean"));
    sys += extra;
    assert!(sys.get_language_by_small_name("pt_br").is_some());
    assert!(sys.get_language_by_small_name("ko_kr").is_some());
}

#[test]
fn language_system_replace_language_by_same_id() {
    let mut sys = LanguageSystem::new();
    sys.add_language(make_language("en_us", "Old Name"));
    sys.add_language(make_language("en_us", "New Name"));
    // Second insert should overwrite
    let lang = sys.get_language_by_small_name("en_us").unwrap();
    assert_eq!(lang.full_name, "New Name");
}

#[test]
fn language_system_set_by_name_unknown_is_noop() {
    let mut sys = LanguageSystem::new();
    sys.add_language(make_language("en_us", "English"));
    sys.set_current_language_by_name("xx_xx"); // unknown — should not panic or change
    assert!(sys.get_current_language().is_none());
}

#[test]
fn language_system_default_equals_new() {
    let sys = LanguageSystem::default();
    assert!(sys.get_current_language().is_none());
}

// ─────────────────────────────────────────────────────────────────────────────
// DictionarySystem
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn dictionary_system_starts_empty() {
    let sys = DictionarySystem::new();
    assert_eq!(sys.get_dictionaries().count(), 0);
}

#[test]
fn dictionary_system_add_and_lookup() {
    let mut sys = DictionarySystem::new();
    let text = "Continue";
    let expected_id = generate_id_from_name(text);
    sys.add_dictionary(text);
    assert!(sys.get_dictionary(expected_id).is_some(), "should find entry");
}

#[test]
fn dictionary_system_add_explicit_entry() {
    let mut sys = DictionarySystem::new();
    sys.add_dictionary_entry(42, "Custom");
    assert!(sys.get_dictionary(42).is_some(), "should find entry by explicit id");
}

#[test]
fn dictionary_system_missing_returns_none() {
    let sys = DictionarySystem::new();
    assert!(sys.get_dictionary(9999).is_none());
}

#[test]
fn dictionary_system_add_operator_merges() {
    let mut a = DictionarySystem::new();
    let mut b = DictionarySystem::new();
    a.add_dictionary("Alpha");
    b.add_dictionary("Beta");
    let merged = a + b;
    assert_eq!(merged.get_dictionaries().count(), 2);
}

#[test]
fn dictionary_system_add_assign_merges() {
    let mut sys = DictionarySystem::new();
    let mut extra = DictionarySystem::new();
    sys.add_dictionary("Hello");
    extra.add_dictionary("World");
    sys += extra;
    assert_eq!(sys.get_dictionaries().count(), 2);
}

#[test]
fn dictionary_partial_eq_by_id() {
    let a = Dictionary::new("same_key");
    let b = Dictionary::new("same_key");
    assert_eq!(a, b, "dictionaries with same key should be equal");
}

#[test]
fn dictionary_partial_eq_different_keys_not_equal() {
    let a = Dictionary::new("key_a");
    let b = Dictionary::new("key_b");
    assert_ne!(a, b, "dictionaries with different keys should not be equal");
}

#[test]
fn dictionary_system_default_equals_new() {
    let sys = DictionarySystem::default();
    assert_eq!(sys.get_dictionaries().count(), 0);
}

#[test]
fn dictionary_system_stress_1000_entries() {
    let mut sys = DictionarySystem::new();
    for i in 0u32..1000 {
        sys.add_dictionary(&format!("entry_{i}"));
    }
    assert_eq!(sys.get_dictionaries().count(), 1000);
    for i in 0u32..1000 {
        let text = format!("entry_{i}");
        let id = generate_id_from_name(&text);
        assert!(
            sys.get_dictionary(id).is_some(),
            "missing entry {i}"
        );
    }
}

