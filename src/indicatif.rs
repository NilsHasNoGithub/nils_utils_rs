

pub use indicatif;
use indicatif::{ProgressStyle, ProgressBar};


pub fn progress_style() -> ProgressStyle {
    ProgressStyle::default_bar().template("[{elapsed_precise}]<[{eta_precise}] {wide_bar} {pos:>7}/{len:7} {msg}").progress_chars("#9876543210-")
}

pub fn progress_bar(count: u64) -> ProgressBar {
    ProgressBar::new(count).with_style(progress_style())
}

#[test]
fn test_progress_bar() {
    use std::time::Duration;
    use indicatif::{ProgressIterator};

    let count = 5_000;

    let pb = progress_bar(count);

    (0..count).progress_with(pb).for_each(|_| {
        std::thread::sleep(Duration::from_millis(1));
    });

}