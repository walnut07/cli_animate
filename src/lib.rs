//! # cli-animate
//! 
//! Enrich your Rust CLIs with stunning, easy-to-implement animations.

use std::sync::{Arc, Mutex};
use std::io::{Write, Stdout};

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
    pub get_progress: Arc<Mutex<dyn Fn() -> u64 + Send>>

    // TODO: Add a new struct to customize style and animation.
    // For example: struct Style {
        // bar_character: char,
        // color: Color,
    // }
}

impl ProgressBar {
    /// `new()` initializes a new progress bar.
    pub fn new<F>(
        start: u64,
        goal: u64,
        get_progress: F,
    ) -> ProgressBar
    where
        F: Fn() -> u64 + Send + 'static,
    {
        assert!(start <= goal);

        ProgressBar {
            start,
            goal,
            get_progress: Arc::new(Mutex::new(get_progress)),
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

        writeln!(writer, "\nFinished").unwrap();
    }

    // NOTE: This function is separated from `start()` just to make it testable.
    // It's impossible to test output to stdout, so we test only this function.
    fn update_display(&self, writer: &mut dyn Write, current_value: u64) {
        let percentage = ((current_value - self.start) as f64 / (self.goal - self.start) as f64) * 100.0;

        let bar_length = 50;
        let completed = ((percentage / 100.0) * bar_length as f64) as usize;
        let bar = "=".repeat(completed) + &" ".repeat(bar_length - completed);

        write!(writer, "\r[{}]", bar).unwrap();
        writer.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_update_progress_success() {
        let progress_bar = ProgressBar::new(0, 100, || 0);
        let mut writer = Cursor::new(Vec::new());

        progress_bar.update_display(&mut writer, 50);

        let expected_output = "\r[=========================                         ]"; // 25 =, 25 ' '.
        assert_eq!(writer.get_ref().as_slice(), expected_output.as_bytes());
    }
}