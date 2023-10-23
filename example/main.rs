use cli_animate::interactive_menu::InteractiveMenu;
use cli_animate::progress::{Color, ProgressBar, Style, StyleBuilder};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    // --------------------------------------------- //
    // ---------- Example 1: Progress Bar ---------- //
    // --------------------------------------------- //

    let progress_value = Arc::new(Mutex::new(0));

    // Create a clone of the progress_value for the other thread.
    let thread_progress_value = progress_value.clone();

    // Some work done in another thread.
    let do_some_work = thread::spawn(move || {
        let mut num = 0;
        while num <= 100 {
            thread::sleep(time::Duration::from_millis(20));

            let mut val = thread_progress_value.lock().unwrap();
            *val = num;

            num += 1;
        }
    });

    // Initialize a progress bar.
    let style = StyleBuilder::new() // `Style` provides builder pattern.
        .color(Color::Green)
        .bar_length(60)
        .build();
    let progress_bar = ProgressBar::new(0, 100, move || *progress_value.lock().unwrap(), style);

    let mut writer = std::io::stdout();

    // Start the progress bar.
    progress_bar.start(&mut writer);

    // Wait for the worker thread to finish.
    do_some_work.join().unwrap();

    // ------------------------------------------------- //
    // ---------- Example 2: Interactive Menu ---------- //
    // ------------------------------------------------- //

    let options = vec![
        "Tokyo".to_string(),
        "Saitama".to_string(),
        "Kanagawa".to_string(),
    ];

    let mut menu = InteractiveMenu::new(options.clone());
    let selected_index = menu.run().unwrap();

    println!("You selected: {}", options[selected_index]);
}
