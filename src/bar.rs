use indicatif::{ProgressBar, ProgressStyle};

pub fn create_bar() -> ProgressBar {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
            )
            .expect("Failed to create progress bar template")
            .progress_chars("#-"),
    );
    pb
}
