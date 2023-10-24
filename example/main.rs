use cli_animate::interactive_menu::{InteractiveMenu, StyleBuilder as InteractiveMenuStyleBuilder};
use cli_animate::loading_indicator::LoadingIndicator;
use cli_animate::progress::{ProgressBar, StyleBuilder as ProgressBarStyleBuilder};
use cli_animate::utils::Color;
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    // --------------------------------------------- //
    // ---------- Example: Progress Bar ------------ //
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
    let style = ProgressBarStyleBuilder::new() // `Style` provides builder pattern.
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
    // ---------- Example: Interactive Menu ------------ //
    // ------------------------------------------------- //

    let options = vec![
        "Tokyo".to_string(),
        "Saitama".to_string(),
        "Kanagawa".to_string(),
    ];

    // Initialize an interactive menu.
    let mut menu = InteractiveMenu::new(
        options.clone(),
        InteractiveMenuStyleBuilder::new() // `Style` provides builder pattern.
            .color(Color::Blue)
            .selected_prefix('*')
            .build(),
    );

    // Run the interactive menu.
    let selected_index = menu.run().unwrap();

    println!("You selected: {}", options[selected_index]);

    // ------------------------------------------------- //
    // ---------- Example: Loading Indicator ----------- //
    // ------------------------------------------------- //

    let loading_indicator = LoadingIndicator::new(Color::Red);
    loading_indicator.start();

    // Do some work while the loading indicator is running.
    let mut num = 0;
    while num <= 100 {
        thread::sleep(time::Duration::from_millis(20));
        num += 1;
    }

    // Stop the loading indicator.
    loading_indicator.stop();
}
