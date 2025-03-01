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

pub fn validate_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if url.is_empty() {
        return Err("Error: URL cannot be empty".into());
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Error: URL must start with http:// or https://".into());
    }
    Ok(())
}
