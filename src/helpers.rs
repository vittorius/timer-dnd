use std::{
    io::{self, Write},
    time::Duration,
};

pub fn clear_line_and_write(content: &str) {
    print!("\r\x1b[2K"); // \r moves the cursor to the beginning, \x1b[2K clears the line
    print!("{}", content);

    // Ensure the output is flushed immediately.
    io::stdout().flush().unwrap();
}

pub trait DurationExt {
    fn to_hms(&self) -> (u64, u64, u64);
}

impl DurationExt for Duration {
    fn to_hms(&self) -> (u64, u64, u64) {
        (
            self.as_secs() / 3600,
            (self.as_secs() % 3600) / 60,
            self.as_secs() % 60,
        )
    }
}
