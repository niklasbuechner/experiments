mod gap;

use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use gap::GapBuffer;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let small_file_content = load_file_contents("/Users/Niklas/Development/Apache/www/experiments/editor-content-management/editor-content-small.txt").unwrap();

    let time = get_timestamp();
    let gap_buffer = GapBuffer::from_str(&small_file_content).unwrap();
    println!("Time used: {}", get_timestamp() - time);
    // Initialize gap buffer

    println!("Gap method!");
    println!("{}", gap_buffer.visualize_gap('§'));
}

fn load_file_contents(url: &str) -> std::io::Result<String> {
    let mut file = File::open(url)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn get_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let timestamp =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    timestamp
}
