use std::io::stdin;
use std::io::{self, Write};

pub struct InteractiveMenu {
    options: Vec<String>,
    selected_index: usize,
}

impl InteractiveMenu {
    /// `new()` initializes a new interactive menu.
    pub fn new(options: Vec<String>) -> Self {
        Self {
            options,
            selected_index: 0,
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
        println!("\x1B[2J\x1B[1;1H\x1b[37m");
        io::stdout().flush()?;

        println!("'w' for up, 's' for down, then press Enter to select. Double Enter to submit.\n");

        for (i, option) in self.options.iter().enumerate() {
            if i == self.selected_index {
                print!("> ");
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
