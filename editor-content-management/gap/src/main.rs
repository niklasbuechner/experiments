mod gap;

use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use gap::GapBuffer;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let small_file_content = load_file_contents("/Users/Niklas/Development/Apache/www/experiments/editor-content-management/editor-content-small.txt").unwrap();

    let time = get_timestamp();
    let mut gap_buffer = GapBuffer::from_str(&small_file_content).unwrap();
    println!("Time used: {}", get_timestamp() - time);

    println!("Gap method!");
    println!("{}", gap_buffer.visualize_gap());

    println!("");
    println!("");

    let time = get_timestamp();
    gap_buffer.insert(0, 5, &mut "_good".to_string());
    println!("Time used: {}", get_timestamp() - time);
    println!("{}", gap_buffer.visualize_gap());

    let time = get_timestamp();
    gap_buffer.insert(0, 5, &mut "_really".to_string());
    println!("Time used: {}", get_timestamp() - time);
    println!("{}", gap_buffer.visualize_gap());

    let time = get_timestamp();
    gap_buffer.insert(0, 5, &mut "_is".to_string());
    println!("Time used: {}", get_timestamp() - time);
    println!("{}", gap_buffer.visualize_gap());

    let time = get_timestamp();
    gap_buffer.insert(0, 0, &mut "<php>".to_string());
    println!("Time used: {}", get_timestamp() - time);
    println!("{}", gap_buffer.visualize_gap());

    let time = get_timestamp();
    gap_buffer.delete(0, 13, 0, 19);
    println!("Time used: {}", get_timestamp() - time);
    println!("{}", gap_buffer.visualize_gap());

    let time = get_timestamp();
    gap_buffer.insert(19, 0, &mut "</php>".to_string());
    println!("Time used: {}", get_timestamp() - time);
    println!("{}", gap_buffer.visualize_gap());


    println!("{}", gap_buffer.to_string());
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