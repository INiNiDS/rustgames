- [] **Design Localization System**
    
    - [x] Select a file format for strings (`Fluent`, `TOML`, or `JSON`).
        
    - [x] Implement a `LocaleManager` resource to handle active language state.
        
- [x] **Backend Implementation (Rust)**
    
    - [x] Create a `Translation` trait for lookups.
        
    - [x] Build a system to hot-reload language files during development.
        
    - [x] Integrate with the `wgpu` text-rendering pipeline (handle UTF-8/Special characters).
                
- [x] **Assets**
    
    - [x] Organize folder structure: `assets/locales/{en-US, ru-RU}/`.
        
    - [x] Ensure fonts support the character sets for all target languages.
**Priority**: Medium