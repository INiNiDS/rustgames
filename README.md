# Rustgames
Rustgames is an engine for making games in Rust. It is built on top of the wgpu graphics API and the winit windowing library. It is designed to be easy to use and to provide a high level of performance.
## Features
- A simple and intuitive API for creating games.
- Support for 2D and 3D graphics. (IN PROGRESS)
- A built-in physics engine. (IN PROGRESS)
- Support for audio. 
- A built-in entity-component-system (ECS) for managing game objects. (IN PROGRESS)
- A built-in input system for handling user input. 
- A built-in UI system for creating user interfaces. (IN PROGRESS)
## Getting Started
To get started with Rustgames, you can add it as a dependency in your Cargo.toml file:
```toml
[dependencies]
rustgames = "path/to/rustgames"
```
Then, you can create a new game by creating a new struct that implements the `Game` trait (Now It's like that, but it will be changed in the: future):
```rust
use rustgames::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn update(&mut self, engine: Engine) {
        // Update game logic here
    }
    fn init(&mut self, engine: Engine) {
        // Initialize game resources here
    }
    fn handle_input(&mut self, engine: Engine) {
        // Handle user input here
    }
}
```
Finally, you can run your game by calling the `run` function:
```rust
fn main() {
    let config = WindowConfig {
        title: "My cool game".into(),
        width: 2560,
        height: 1440,
        resizable: true,
        fullscreen: true,
        vsync: true,
        background_color: Color::new(0.05, 0.0, 0.1, 1.0),
        language: Language::resolve("en_us").unwrap(),
    };
    app::run(config, Box::new(MyGame)).expect("Failed to run");
}
```
## Contributing
Contributions to Rustgames are welcome! If you have an idea for a new feature or have found a bug, please open an issue or submit a pull request. Please make sure to follow the contribution guidelines and to write tests for any new features or bug fixes.
## License
Rustgames is licensed under the MIT OR APACHE 2.0 License. See the LICENSE-MIT OR LICENSE-APACHE file for more information.