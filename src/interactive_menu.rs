use std::io::stdin;
use std::io::{self, Write};

use crate::utils::Color;

pub struct InteractiveMenu {
    /// Options to be displayed.
    options: Vec<String>,

    /// The index of the selected option.
    selected_index: usize,

    /// The style of the interactive menu.
    style: Style,
}

/// The style of the interactive menu.
pub struct Style {
    color: Color,

    /// The prefix of the selected option, such as `>`, `*`, etc.
    selected_prefix: char,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            color: Color::White,
            selected_prefix: '>',
        }
    }
}

pub struct StyleBuilder {
    color: Option<Color>,
    selected_prefix: Option<char>,
}

impl StyleBuilder {
    pub fn new() -> StyleBuilder {
        StyleBuilder {
            color: None,
            selected_prefix: None,
        }
    }

    pub fn color(mut self, color: Color) -> StyleBuilder {
        self.color = Some(color);
        self
    }

    pub fn selected_prefix(mut self, prefix: char) -> StyleBuilder {
        self.selected_prefix = Some(prefix);
        self
    }

    pub fn build(self) -> Style {
        Style {
            color: self.color.unwrap_or(Color::White),
            selected_prefix: self.selected_prefix.unwrap_or('>'),
        }
    }
}

impl InteractiveMenu {
    /// `new()` initializes a new interactive menu.
    pub fn new(options: Vec<String>, style: Style) -> Self {
        Self {
            options,
            selected_index: 0,
            style,
        }
    }

    /// `run()` starts the interactive menu.
    /// It ends when the user submits an option.
    pub fn run(&mut self) -> io::Result<usize> {
        self.display()?;

        loop {
            let mut input = String::new();
            stdin().read_line(&mut input)?;

            match input.trim() {
                "w" | "W" => self.previous(), // 'w' to move up
                "s" | "S" => self.next(),     // 's' to move down
                "" => break,                  // Enter key to select
                _ => continue,
            }

            self.display()?;
        }

        Ok(self.selected_index)
    }

    /// `display()` displays the interactive menu where there are options.
    fn display(&mut self) -> io::Result<()> {
        // \x1b[2J clears the screen. x1B[1;1H sets screen size 40 x 25. \x1b[37m sets color as white.
        println!("\x1B[2J\x1B[1;1H{}", self.style.color.to_ansi_code());
        io::stdout().flush()?;

        println!("'w' for up, 's' for down, then press Enter to select. Double Enter to submit.\n");

        for (i, option) in self.options.iter().enumerate() {
            if i == self.selected_index {
                print!("{} ", self.style.selected_prefix);
            } else {
                print!("  ");
            }

            println!("{}", option);
        }

        io::stdout().flush()?;
        Ok(())
    }

    fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.options.len() - 1;
        }
    }

    fn next(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.options.len();
    }
}
