# 🛠 Engine Plan: Custom Builder CLI

## 1. CLI Architecture
- [ ] Initialize a new Rust binary crate (e.g., `engine-cli`).
- [ ] Integrate `clap` for parsing command-line arguments and subcommands.
- [ ] Define core subcommands: `build`, `run`, `pack`, `deploy`.

## 2. Build Automation
- [ ] Implement the `build` command to wrap `cargo build` with specific feature flags.
- [ ] Add cross-compilation support (e.g., passing `--target` for Android or Windows builds).
- [ ] Integrate shader compilation (automatically converting GLSL/HLSL to SPIR-V or WGSL before building).

## 3. Asset Packaging
- [ ] Create a `pack` command to compress and bundle assets (textures, audio, fonts) into a custom binary format or archive.
- [ ] Implement an asset watcher that triggers a rebuild or hot-reload when source files change.

## 4. Deployment & Execution
- [ ] Implement the `deploy` command for Android (wrapping `adb install` and `adb shell am start`).
- [ ] Implement a unified log reader (combining standard stdout with `adb logcat` when running on mobile).