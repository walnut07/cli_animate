use std::sync::{Arc, Mutex};
use cli_animate::ProgressBar;
use std::{thread, time};

fn main() {
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

    // Initialize a new ProgressBar the initial number of steps and the goal.
    let progress_bar = ProgressBar::new(
        0,
        100,
        move || *progress_value.lock().unwrap(),
    );

    let mut writer = std::io::stdout();

    // Start the progress bar.
    progress_bar.start(&mut writer);

    // Wait for the worker thread to finish.
    do_some_work.join().unwrap();
}