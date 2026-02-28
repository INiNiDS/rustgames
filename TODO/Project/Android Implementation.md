# 📱 Engine Plan: Android Porting

## 1. Environment & Toolchain Setup
- [ ] Install Android Studio (SDK, NDK, Command-line Tools).
- [ ] Add Rust Android targets: `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`.
- [ ] Install build utilities like `cargo-ndk`.
- [ ] Export `ANDROID_NDK_HOME` to environment variables.

## 2. Project Configuration (`Cargo.toml`)
- [ ] Configure `[package.metadata.android]` for APK generation (package name, permissions, icons).
- [ ] Add Android-specific dependencies behind `target_os = "android"`:
  - `android_logger` for Logcat integration.
  - `android-activity` or `ndk-sys` for the application backend.
- [ ] Verify `winit` and `wgpu` versions for latest Android backend compatibility.

## 3. Entry Point & Lifecycle Management
- [ ] Replace standard `main()` with the `#[no_mangle] fn android_main(app: android_activity::AndroidApp)` entry point.
- [ ] Integrate `AndroidApp` into the `winit` event loop.
- [ ] **Critical:** Handle OS lifecycle events:
  - `Resumed`: Initialize or restore the graphics context (create `wgpu` Surface).
  - `Suspended`: Destroy the `Surface` (Android invalidates the context when minimized).

## 4. Graphics & Asset Pipeline
- [ ] Ensure the `wgpu` backend defaults to Vulkan (or OpenGL ES as a fallback).
- [ ] Delay `Surface` creation until the `Resumed` event is fired by `winit`.
- [ ] Implement asset loading via `AAssetManager` instead of `std::fs` to read shaders, models, and textures directly from the APK.

## 5. Input Handling
- [ ] Map Android touch events to engine input actions.
- [ ] Implement soft keyboard invocation for text input fields.