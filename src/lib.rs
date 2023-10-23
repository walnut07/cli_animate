//! # cli-animate
//!
//! `cli-animate` is a Rust crate designed to **enrich command-line applications with a variety of beautiful, easy-to-use animations**.
//! It offers a straightforward way to integrate visual elements such as progress bars, interactive menus, and more, enhancing the interactivity of your CLIs.
//!
//! ## Features
//!
//! - **Progress Bars**: Show task progress with customizable, animated progress bars.
//! - **Interactive Menus**: Navigate through options with intuitive, keyboard-navigable menus.
//! - **Loading Indicators**: Display a loading indicator to show that your application is working.
//! - **And More**: The library is designed for extensibility and includes a variety of other tools to enrich your CLI applications.
//!
//! ## Quick Start
//!
//! Add `cli-animate` to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! cli-animate = "0.1.0"
//! ```
//!
//! Here's a simple example of how to use `cli-animate` to create an interactive menu:
//!
//! ```rust
//! use cli_animate::interactive_menu::InteractiveMenu;
//!
//! fn main() {
//!     // Options to display in the user's terminal.
//!     let options = vec![
//!       "Tokyo".to_string(),
//!       "Saitama".to_string(),
//!       "Kanagawa".to_string(),
//!      ];
//!
//!     // Initialize an interactive menu.
//!     let mut menu = InteractiveMenu::new(options.clone());
//!
//!     // Run the interactive menu.
//!     let selected_index = menu.run().unwrap();
//!
//!     // User is seeing something like this in their terminal:
//!     //
//!     // 'w' for up, 's' for down, then press Enter to select. Double Enter to submit.
//!     //
//!     // > Tokyo
//!     //   Saitama
//!     //   Kanagawa
//!
//!    // When the user presses Enter twice, the selected option is returned.
//!     println!("You selected: {}", options[selected_index]);
//! }
//! ```
//! For more examples, see the folder `examples/` in the [cli-animate GitHub repository](https://github.com/walnut07/cli_animate)!
//!
//! ## Feedback and Contributions
//!
//! We welcome your feedback and contributions! If you find an issue or have a suggestion for improvement, please file an issue on the [cli-animate GitHub repository](https://github.com/your-github-username/cli-animate).
//!
//! Thank you for using `cli-animate` to make your command-line applications more attractive and engaging!

pub mod interactive_menu;
pub mod loading_indicator;
pub mod progress;
pub mod utils;
