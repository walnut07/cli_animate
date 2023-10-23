use std::default::Default;
use std::io::{Stdout, Write};
use std::sync::{Arc, Mutex};

/// A `ProgressBar` handles the animation of a progress bar.
pub struct ProgressBar {
    /// The start value of the progress.
    /// The `start` and `goal` values must be absolute values, not relative to each other.
    /// For example, if your ideal state is 700 and your current state is 500, set `start` to 500 and `goal` to 700.
    /// Vice versa, if you want to start at 50% of the progress, set `start` to 50 and `goal` to 100.
    pub start: u64,

    /// The goal value of the progress. It must be an absolute value, not relative to the start.
    pub goal: u64,

    /// A closure to get the current progress value.
    pub get_progress: Arc<Mutex<dyn Fn() -> u64 + Send>>,

    pub style: Style,
}

/// A `Style` defines the appearance of a progress bar.
pub struct Style {
    /// The character used to display the progress bar, such as `=`, `#`, `*`, etc.
    pub bar_character: char,

    /// The length of the progress bar in characters.
    pub bar_length: u64,

    /// The color of the progress bar. It will be printed as a 24 bit color.
    pub color: Color,
}

#[derive(Default)]
pub enum Color {
    #[default]
    White,
    Red,
    Blue,
    Yellow,
    Green,
    Cyan,
    Magenta,
    Black,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            bar_character: '=',
            bar_length: 50,
            color: Color::White,
        }
    }
}

pub struct StyleBuilder {
    bar_character: Option<char>,
    bar_length: Option<u64>,
    color: Option<Color>,
}

impl StyleBuilder {
    pub fn new() -> StyleBuilder {
        StyleBuilder {
            bar_character: None,
            bar_length: None,
            color: None,
        }
    }

    pub fn bar_character(mut self, character: char) -> StyleBuilder {
        self.bar_character = Some(character);
        self
    }

    pub fn bar_length(mut self, length: u64) -> StyleBuilder {
        self.bar_length = Some(length);
        self
    }

    pub fn color(mut self, color: Color) -> StyleBuilder {
        self.color = Some(color);
        self
    }

    pub fn build(self) -> Style {
        Style {
            bar_character: self.bar_character.unwrap_or('='),
            bar_length: self.bar_length.unwrap_or(50),
            color: self.color.unwrap_or(Color::White),
        }
    }
}

impl Color {
    pub fn to_ansi_code(&self) -> &'static str {
        match self {
            Color::White => "\x1b[37m",
            Color::Red => "\x1b[31m",
            Color::Blue => "\x1b[34m",
            Color::Yellow => "\x1b[33m",
            Color::Green => "\x1b[32m",
            Color::Cyan => "\x1b[36m",
            Color::Magenta => "\x1b[35m",
            Color::Black => "\x1b[30m",
        }
    }
}

impl ProgressBar {
    /// `new()` initializes a new progress bar.
    pub fn new<F>(start: u64, goal: u64, get_progress: F, style: Style) -> ProgressBar
    where
        F: Fn() -> u64 + Send + 'static,
    {
        assert!(start <= goal);

        ProgressBar {
            start,
            goal,
            get_progress: Arc::new(Mutex::new(get_progress)),
            style,
        }
    }

    /// `start()` starts the animation of the progress bar.
    /// It displays from 0% and goes to 100%.
    pub fn start(&self, writer: &mut Stdout) {
        let mut current_value = self.start;

        while current_value < self.goal {
            current_value = (self.get_progress.lock().unwrap())();
            self.update_display(writer, current_value);
        }
    }

    // NOTE: This function is separated from `start()` just to make it testable.
    // It's impossible to test output to stdout, so we test only this function.
    fn update_display(&self, writer: &mut dyn Write, current_value: u64) {
        let percentage =
            ((current_value - self.start) as f64 / (self.goal - self.start) as f64) * 100.0;

        let bar_length = self.style.bar_length as usize;
        let completed = ((percentage / 100.0) * bar_length as f64) as usize;
        let bar = self.style.bar_character.to_string().repeat(completed)
            + &" ".repeat(bar_length - completed);

        let color_code = self.style.color.to_ansi_code();

        write!(writer, "\r{}[{}]", &color_code, bar).unwrap();

        writer.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_update_progress_success() {
        let progress_bar = ProgressBar::new(0, 100, || 0, Style::default());
        let mut writer = Cursor::new(Vec::new());

        progress_bar.update_display(&mut writer, 50);

        let expected_output = "\r\x1b[37m[=========================                         ]"; // 25 =, 25 ' '.
        assert_eq!(writer.get_ref().as_slice(), expected_output.as_bytes());
    }
}
