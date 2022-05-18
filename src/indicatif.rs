

pub use indicatif;
use indicatif::{ProgressStyle, ProgressBar};

#[static_init::dynamic]
static PROGRESS_STYLE: ProgressStyle = ProgressStyle::default_bar().template("[{elapsed_precise}]<[{eta_precise}] {wide_bar} {pos:>7}/{len:7} {msg}").progress_chars("#9876543210-");


pub fn progress_bar(count: u64) -> ProgressBar {
    ProgressBar::new(count).with_style(PROGRESS_STYLE.clone())
}

#[test]
fn test_progress_bar() {
    use std::time::Duration;
    use indicatif::{ProgressBar,ProgressIterator};

    let count = 5_000;

    let pb = ProgressBar::new(count).with_style(PROGRESS_STYLE.clone());

    (0..count).progress_with(pb).for_each(|_| {
        std::thread::sleep(Duration::from_millis(1));
    });

}