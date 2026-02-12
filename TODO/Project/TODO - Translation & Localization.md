- [ ] **Design Localization System**
    
    - [ ] Select a file format for strings (`Fluent`, `TOML`, or `JSON`).
        
    - [ ] Implement a `LocaleManager` resource to handle active language state.
        
- [ ] **Backend Implementation (Rust)**
    
    - [ ] Create a `Translation` trait for lookups.
        
    - [ ] Build a system to hot-reload language files during development.
        
    - [ ] Integrate with the `wgpu` text-rendering pipeline (handle UTF-8/Special characters).
        
- [ ] **UI Integration**
    
    - [ ] Implement a macro or helper (e.g., `tr!("start_game")`) for quick lookups in code.
        
    - [ ] Add support for "fallback" languages if a key is missing.
        
- [ ] **Assets**
    
    - [ ] Organize folder structure: `assets/locales/{en-US, ru-RU}/`.
        
    - [ ] Ensure fonts support the character sets for all target languages.
**Priority**: Medium