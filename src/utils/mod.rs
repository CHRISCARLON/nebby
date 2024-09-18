use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn create_progress_bar(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_bar()
            .tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷")
            .template("{spinner:.bright_yellow} {msg:.bright_yellow}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}
